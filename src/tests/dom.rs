
use crate::dom::*;

#[test]
fn element() {
    let e = Element::new("test");
    assert_eq!(e.name, "test");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 0);
}

#[test]
fn add_element() {
    let mut e = Element::new("test");

    e.add_element(Element::new("test2"));

    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test2");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn add_text() {
    let mut e = Element::new("test");

    e.add_text("test2");

    assert_eq!(e.nodes.len(), 1);

    if let &Node::Text(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "test2");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn add_comment() {
    let mut e = Element::new("test");

    e.add_comment("test2");

    assert_eq!(e.nodes.len(), 1);

    if let &Node::Comment(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "test2");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn add_cdata() {
    let mut e = Element::new("test");

    e.add_cdata("test2");

    assert_eq!(e.nodes.len(), 1);

    if let &Node::CData(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "test2");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn get_attribute_value() {
    let mut e = Element::new("test");
    e.set_attribute("attr1", "v1");
    e.set_attribute("attr2", "v2");
    e.set_bool_attribute("attr3");

    assert_eq!(e.get_attribute_value("attr1"), Some("v1"));
    assert_eq!(e.get_attribute_value("attr2"), Some("v2"));
    assert_eq!(e.get_attribute_value("attr3"), None);
    assert_eq!(e.get_attribute_value("notexists"), None);

    assert!(e.has_attribute("attr1"));
    assert!(e.has_attribute("attr2"));
    assert!(e.has_attribute("attr3"));
    assert!(!e.has_attribute("notexists"));
}