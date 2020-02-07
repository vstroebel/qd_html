
use crate::dom::*;
use crate::utils::*;

pub struct HtmlWriter<'a> {
    out: &'a mut String,
}

impl<'a> HtmlWriter<'a> {
    pub fn new(out: &'a mut String) -> HtmlWriter {
        HtmlWriter { out }
    }

    pub fn push(&mut self, ch: char) {
        self.out.push(ch);
    }

    pub fn push_str(&mut self, s: &str) {
        self.out.push_str(s);
    }

    pub fn write_doctype(&mut self, dtd: &str) {
        self.push_str("<!DOCTYPE ");
        self.push_str(dtd);
        self.push('>');
    }

    pub fn element_start(&mut self, element: &Element) {
        self.element_start_name(&element.name);

        for attr in &element.attributes {
            self.element_attribute(&attr.name, &attr.value);
        }

        self.push('>');
    }

    pub fn element_start_name(&mut self, name: &str) {
        self.push('<');
        self.push_str(name);
    }

    pub fn element_attribute(&mut self, name: &str, value: &Option<String>) {
        self.push(' ');
        self.push_str(name);

        if let Some(ref value) = *value {
            self.push('=');
            self.push('"');
            self.append_text(&value, true);
            self.push('"');
        }
    }

    pub fn element_end(&mut self, element: &Element) {
        self.element_end_for_name(&element.name);
    }

    pub fn element_end_for_name(&mut self, element_name: &str) {
        self.push('<');
        self.push('/');
        self.push_str(element_name);
        self.push('>');
    }

    pub fn append_text(&mut self, text: &str, attribute: bool) {
        for ch in text.chars() {
            match ch {
                '&' => self.push_str("&amp;"),
                '>' => {
                    if attribute {
                        self.push(ch);
                    } else {
                        self.push_str("&gt;");
                    }
                }
                '<' => {
                    if attribute {
                        self.push(ch);
                    } else {
                        self.push_str("&lt;");
                    }
                }
                '"' => {
                    if attribute {
                        self.push_str("&quot;");
                    } else {
                        self.push(ch);
                    }
                }
                '\u{a0}' => self.push_str("&nbsp;"),
                '\n' | '\t' | '\r' | ' ' => self.push(ch),
                '\u{0}'..='\u{1f}' => {
                    //Ignore ISO control chars
                }
                _ => {
                    if ch.is_whitespace() {
                        self.push_str(&format!("&x{};", ch as u32));
                    } else {
                        self.push(ch);
                    }
                }
            }
        }
    }

    pub fn append_comment(&mut self, content: &str) {
        self.push_str("<!--");
        self.push_str(content);
        self.push_str("-->");
    }

    pub fn append_cdata(&mut self, content: &str) {
        self.push_str("<![CDATA[");
        self.push_str(content);
        self.push_str("]]>");
    }

    pub fn append_raw(&mut self, content: &str) {
        self.push_str(content);
    }
}

pub fn write(doc: &Document) -> String {
    let mut result = "".to_owned();
    {
        let mut writer = HtmlWriter::new(&mut result);
        if let Some(ref dtd) = doc.doctype {
            writer.write_doctype(dtd);
        }
        append_nodes(&mut writer, &doc.element.nodes);
    }
    result
}

pub fn write_element(root: &Element) -> String {
    let mut result = "".to_owned();
    {
        let mut writer = HtmlWriter::new(&mut result);

        if root.name == "#document" {
            append_nodes(&mut writer, &root.nodes);
        } else {
            append_element(&mut writer, root);
        }
    }
    result
}

fn append_nodes(mut writer: &mut HtmlWriter, nodes: &[Node]) {
    for node in nodes {
        match *node {
            Node::Element(ref e) => append_element(&mut writer, e),
            Node::Text(ref text) => writer.append_text(&text.content, false),
            Node::Comment(ref comment) => writer.append_comment(&comment.content),
            Node::CData(ref cdata) => writer.append_cdata(&cdata.content),
            Node::Raw(ref raw) => writer.append_raw(&raw.content),            
        }
    }
}

fn append_element(writer: &mut HtmlWriter, element: &Element) {
    writer.element_start(element);
    if !is_autoclose_element(&element.name) {
        append_nodes(writer, &element.nodes);
        writer.element_end(element);
    }
}
