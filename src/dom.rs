//! Minimalistic Document Object Model

/// A node
#[derive(Debug)]
pub enum Node {
    Text(Text),
    Comment(Comment),
    Element(Element),
    Raw(Raw),
    CData(CData),
    ProcessingInstruction(ProcessingInstruction)
}

/// Text node
#[derive(Debug)]
pub struct Text {
    pub content: String,
}

impl Text {
    pub fn new<T: Into<String>>(content: T) -> Text {
        Text { content: content.into() }
    }
}


/// Comment node
#[derive(Debug)]
pub struct Comment {
    pub content: String,
}

impl Comment {
    pub fn new<T: Into<String>>(content: T) -> Comment {
        Comment { content: content.into() }
    }
}

/// Raw node
#[derive(Debug)]
pub struct Raw {
    pub content: String,
}

impl Raw {
    pub fn new<T: Into<String>>(content: T) -> Raw {
        Raw { content: content.into() }
    }
}

/// CData node
#[derive(Debug)]
pub struct CData {
    pub content: String,
}

impl CData {
    pub fn new<T: Into<String>>(content: T) -> CData {
        CData { content: content.into() }
    }
}

/// XML processing instruction
#[derive(Debug)]
pub struct ProcessingInstruction {
    pub content: String,
}

impl ProcessingInstruction {
    pub fn new<T: Into<String>>(content: T) -> ProcessingInstruction {
        ProcessingInstruction { content: content.into() }
    }
}

/// Attribute of an element
#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

impl Attribute {
    pub fn new<T: Into<String>, T2: Into<String>>(name: T, value: T2) -> Attribute {
        Attribute {
            name: name.into(),
            value: Some(value.into()),
        }
    }

    pub fn new_bool<T: Into<String>>(name: T) -> Attribute {
        Attribute {
            name: name.into(),
            value: None,
        }
    }

    pub fn is_bool(&self) -> bool {
        self.value.is_none()
    }

    pub fn destruct(self) -> (String, Option<String>) {
        (self.name, self.value)
    }
}

/// List of attributes of an element
#[derive(Debug, Default)]
pub struct Attributes {
    list: Vec<Attribute>,
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes { list: Vec::new() }
    }

    pub fn set<T: Into<String>, T2: Into<String>>(&mut self, name: T, value: T2) {
        let value = value.into();
        let name = name.into();

        for attr in &mut self.list {
            if attr.name == name {
                attr.value = Some(value);
                return;
            }
        }

        self.list.push(Attribute::new(name, value));
    }

    pub fn set2(&mut self, name: &str, value: String) {

        for attr in &mut self.list {
            if attr.name == name {
                attr.value = Some(value);
                return;
            }
        }

        //let name = name.into();

        self.list.push(Attribute::new(name, value));
    }

    pub fn set_bool<T: Into<String>>(&mut self, name: T) {
        let name = name.into();

        for attr in &mut self.list {
            if attr.name == name {
                attr.value = None;
                return;
            }
        }

        self.list.push(Attribute::new_bool(name));
    }

    pub fn get_value<'a>(&'a self, name: &str) -> Option<&'a str> {
        for attr in &self.list {
            if attr.name == name {
                return attr.value.as_ref().map(|x| x as _);
            }
        }
        None
    }

    pub fn contains(&self, name: &str) -> bool {
        for attr in &self.list {
            if attr.name == name {
                return true;
            }
        }
        false
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn into_vec(self) -> Vec<Attribute> {
        self.list
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = &'a Attribute;
    type IntoIter = ::std::slice::Iter<'a, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.iter()
    }
}

/// Element node
#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub attributes: Attributes,
    pub nodes: Vec<Node>,
    pub autoclosed: bool,
}

impl Element {
    pub fn new<T: Into<String>>(name: T) -> Element {
        Element {
            name: name.into(),
            attributes: Attributes::new(),
            nodes: Vec::new(),
            autoclosed: false,
        }
    }

    pub fn new_with_attributes<T: Into<String>>(name: T, attributes: Attributes) -> Element {
        Element {
            name: name.into(),
            attributes,
            nodes: Vec::new(),
            autoclosed: false,
        }
    }

    pub fn add_element(&mut self, e: Element) {
        self.nodes.push(Node::Element(e));
    }

    pub fn add_text<T: Into<String>>(&mut self, content: T) {
        self.nodes.push(Node::Text(Text::new(content.into())));
    }

    pub fn add_comment<T: Into<String>>(&mut self, content: T) {
        self.nodes.push(Node::Comment(Comment::new(content.into())));
    }

    pub fn add_cdata<T: Into<String>>(&mut self, content: T) {
        self.nodes.push(Node::CData(CData::new(content.into())));
    }

    pub fn add_processing_instruction<T: Into<String>>(&mut self, content: T) {
        self.nodes.push(Node::ProcessingInstruction(ProcessingInstruction::new(content.into())));
    }

    pub fn set_attribute<T: Into<String>, T2: Into<String>>(&mut self, name: T, value: T2) {
        self.attributes.set(name, value);
    }

    pub fn set_bool_attribute<T: Into<String>>(&mut self, name: T) {
        self.attributes.set_bool(name);
    }

    pub fn get_attribute_value<'a>(&'a self, name: &str) -> Option<&'a str> {
        self.attributes.get_value(name)
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains(name)
    }
}

/// Document
#[derive(Debug)]
pub struct Document {
    pub doctype: Option<String>,
    pub is_xml: bool,
    pub element: Element,
}

impl Document {

    /// Creates a new empty HTML document
    #[allow(clippy::new_without_default)]
    pub fn new() -> Document {
        Document {
            doctype: Some("html".to_owned()),
            is_xml: false,
            element: Element::new("#document")
        }
    }
}
