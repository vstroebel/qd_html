use crate::dom::*;
use crate::utils::*;
use crate::reader::*;

/**
  A Parser to parse HTML alike tag soup
*/
struct Parser<'a, H: 'a + ParseHandler> {
    pub handler: &'a mut H,
    pub reader: &'a mut Reader<'a>,
}

impl<'a, H: ParseHandler> Parser<'a, H> {
    pub fn parse(mut self) {
        while let Some(ch) = self.reader.next_char() {
            self.process(ch);
        }

        self.handler.finish();
    }

    fn process(&mut self, ch: char) {
        match ch {
            '<' => {
                match self.reader.next_char() {
                    Some('/') => self.parse_end_element(),
                    Some('!') => {
                        match (self.reader.next_char(), self.reader.next_char()) {
                            (Some('-'), Some('-')) => self.parse_comment(),
                            (Some('['), Some('C')) => self.parse_cdata(),
                            (Some('D'), Some('O')) => self.parse_doctype(),
                            (Some('>'), ch2) => self.parse_unknown_special_tag(ch2),
                            (Some(ch), Some('>')) => self.parse_unknown_special_tag2(ch),
                            (ch, ch2) => self.parse_unknown_special_tag3(ch, ch2),
                        }
                    }
                    Some('?') => self.parse_processing_instruction(),
                    Some(ch) => {
                        self.reader.push_back(ch);
                        self.parse_start_element();
                    }
                    None => self.handler.text("<".into()),
                }
            }
            _ => {
                self.reader.push_back(ch);
                let text = self.reader.read_text(&['<']);
                self.handler.text(text);
            }
        }
    }

    fn parse_doctype(&mut self) {
        let marker = self.reader.read_raw_count(5);
        if &marker == "CTYPE" {
            self.handler.doctype(
                self.reader.read_raw(">").trim().to_owned(),
            );
        } else {
            let mut text = self.reader.read_text(&['<', '>']);
            text.insert_str(0, "<!DO");
            text.insert_str(4, &marker);
            if self.reader.ignore_if_next('>') {
                text.push('>');
            }
            self.handler.text(text);
        }
    }

    fn parse_cdata(&mut self) {
        let marker = self.reader.read_raw_count(5);
        if &marker == "DATA[" {
            let content = self.reader.read_raw("]]>");
            self.handler.cdata(content);
        } else {
            let mut text = self.reader.read_text(&['<', '>']);
            text.insert_str(0, "<![C");
            text.insert_str(4, &marker);
            if self.reader.ignore_if_next('>') {
                text.push('>');
            }
            self.handler.text(text);
        }
    }

    fn parse_comment(&mut self) {
        let content = self.reader.read_raw("-->");
        self.handler.comment(content);
    }

    fn parse_unknown_special_tag(&mut self, ch2: Option<char>) {
        self.handler.text("<!>".to_owned());
        if let Some(ch2) = ch2 {
            self.reader.push_back(ch2);
        }
    }

    fn parse_unknown_special_tag2(&mut self, ch: char) {
        self.handler.text(format!("<!{}>", ch))
    }

    fn parse_unknown_special_tag3(&mut self, ch: Option<char>, ch2: Option<char>) {
        let mut text = "<!".to_owned();

        if let Some(ch) = ch {
            text.push(ch);
        }
        if let Some(ch2) = ch2 {
            text.push(ch2);
            text.push_str(&self.reader.read_text(&['<', '>']));
        }

        if self.reader.ignore_if_next('>') {
            text.push('>');
        }
        self.handler.text(text);
    }


    fn parse_processing_instruction(&mut self) {
        let text = self.reader.read_raw("?>");
        self.handler.processing_instruction(text);
    }

    fn parse_start_element(&mut self) {
        self.reader.skip_whitespace();

        let raw_name = self.reader.get_until(true, &['>', '/']);
        let name = raw_name.to_ascii_lowercase();
        let autoclose = is_autoclose_element(&name);
        let mut attributes = Attributes::new();

        self.reader.skip_whitespace();

        while let Some(ch) = self.reader.next_char() {
            if ch == '/' || ch == '>' {
                self.reader.push_back(ch);
                break;
            }

            if is_html_whitespace(ch) {
                self.reader.skip_whitespace();
                continue;
            }

            self.reader.push_back(ch);

            let name = self.reader.get_until(true, &['=', '/', '>']);
            self.reader.skip_whitespace();

            match self.reader.next_char() {
                Some('=') => {
                    self.reader.skip_whitespace();
                    let value = match self.reader.next_char() {
                        Some('"') => self.reader.read_text(&['"']),
                        Some('\'') => self.reader.read_text(&['\'']),
                        Some(ch) => {
                            self.reader.push_back(ch);
                            self.reader.read_text(&[' ', '/', '>'])
                        }
                        None => {
                            //bad attribute at end of input
                            attributes.set_bool(name);
                            break;
                        }
                    };
                    self.reader.next_char();
                    attributes.set(name, value);
                }
                Some(ch) => {
                    //boolean attribute
                    attributes.set_bool(name);
                    self.reader.push_back(ch);
                }
                None => {
                    //Onclosed element at end of input
                    break;
                }
            }
        }

        let autoclose = match self.reader.next_char() {
            Some('/') => {
                self.reader.next_char();
                true
            }
            _ => autoclose,
        };

        let raw_content = if !autoclose && is_raw_element(&name) {
            Some(self.reader.read_raw(&format!("</{}>", raw_name)))
        } else {
            None
        };

        self.handler.element_start(
            name,
            attributes,
            autoclose,
            raw_content,
        );
    }

    fn parse_end_element(&mut self) {
        self.reader.skip_whitespace();
        let name = self.reader.get_until(true, &['>']).to_ascii_lowercase();
        self.reader.skip_whitespace();
        self.reader.next_char();

        self.handler.element_end(&name);
    }
}

pub trait ParseHandler {
    fn finish(&mut self) {}

    fn text(&mut self, text: String);

    fn comment(&mut self, content: String);

    fn cdata(&mut self, content: String);

    fn doctype(&mut self, content: String);

    fn processing_instruction(&mut self, content: String);

    fn element_start(
        &mut self,
        name: String,
        attributes: Attributes,
        autoclose: bool,
        raw_content: Option<String>,
    );

    fn element_end(&mut self, name: &str);
}

struct DomParseHandler {
    pub stack: Vec<Element>,
    pub current: Element,
    pub doctype: Option<String>,
    pub is_xml: bool,
}

impl ParseHandler for DomParseHandler {
    fn finish(&mut self) {
        while let Some(mut e) = self.stack.pop() {
            ::std::mem::swap(&mut self.current, &mut e);
            self.current.nodes.push(Node::Element(e));
        }
    }

    fn text(&mut self, content: String) {
        self.current.add_text(content);
    }

    fn comment(&mut self, content: String) {
        self.current.add_comment(content);
    }

    fn cdata(&mut self, content: String) {
        self.current.add_cdata(content);
    }

    fn doctype(&mut self, content: String) {
        self.doctype = Some(content);
    }

    fn processing_instruction(&mut self, content: String) {
        if content.starts_with("xml") {
            self.is_xml = true;
        } else {
            self.current.add_processing_instruction(content);
        }
    }

    fn element_start(
        &mut self,
        name: String,
        attributes: Attributes,
        autoclose: bool,
        raw_content: Option<String>,
    ) {
        let mut e = Element::new_with_attributes(name, attributes);
        e.autoclosed = autoclose;

        if autoclose {
            self.current.add_element(e);
        } else if let Some(raw_content) = raw_content {
            e.nodes.push(Node::Raw(Raw::new(raw_content)));
            self.current.nodes.push(Node::Element(e));
        } else {
            ::std::mem::swap(&mut self.current, &mut e);
            self.stack.push(e);
        }
    }

    fn element_end(&mut self, name: &str) {
        while let Some(mut e) = self.stack.pop() {
            ::std::mem::swap(&mut self.current, &mut e);

            let same = e.name == name;
            self.current.add_element(e);
            if same {
                return;
            }
        }
    }
}

pub fn parse<H: ParseHandler>(handler: &mut H, raw: &str) {
    Parser {
        handler,
        reader: &mut Reader::new(raw),
    }.parse();
}

/// Parse HTML and build a simplified DOM tree
pub fn parse_to_dom(raw: &str) -> Document {
    let mut handler = DomParseHandler {
        stack: Vec::new(),
        current: Element::new("#document"),
        doctype: None,
        is_xml: false,
    };

    parse(&mut handler, raw);

    Document {
        doctype: handler.doctype,
        is_xml: handler.is_xml,
        element: handler.current,
    }
}
