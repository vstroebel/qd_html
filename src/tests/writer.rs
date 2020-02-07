
use crate::writer::*;
use crate::dom::*;

#[test]
fn empty() {
    assert_eq!("", write_element(&Element::new("#document")));
}

#[test]
fn empty_doc() {
    assert_eq!(
        "",
        write(&Document {
            doctype: None,
            element: Element::new("#document"),
        })
    );
}

#[test]
fn doctype() {

    assert_eq!(
        "<!DOCTYPE html>",
        write(&Document {
            doctype: Some("html".to_owned()),
            element: Element::new("#document"),
        })
    );
}

#[test]
fn single_element() {
    let mut doc = Element::new("#document");
    doc.nodes.push(Node::Element(Element::new("test")));
    assert_eq!("<test></test>", write_element(&doc));
}

#[test]
fn single_element_with_attributes() {
    let mut doc = Element::new("#document");
    let mut e = Element::new("test");

    e.attributes.set("attr1", "v1");
    e.attributes.set_bool("attr2");

    doc.nodes.push(Node::Element(e));

    assert_eq!("<test attr1=\"v1\" attr2></test>", write_element(&doc));
}

#[test]
fn single_element_with_attribute_and_entity() {
    let mut doc = Element::new("#document");
    let mut e = Element::new("test");

    e.attributes.set("attr1", "><&\"'\u{a0}äöü");
    
    doc.nodes.push(Node::Element(e));
    assert_eq!(
        "<test attr1=\"><&amp;&quot;'&nbsp;äöü\"></test>",
        write_element(&doc)
    );
}

#[test]
fn text() {
    let mut doc = Element::new("#document");
    let e = Text::new("><&\"'\u{a0}äöü");
    doc.nodes.push(Node::Text(e));
    assert_eq!("&gt;&lt;&amp;\"'&nbsp;äöü", write_element(&doc));
}

#[test]
fn single_comment() {
    let mut doc = Element::new("#document");
    doc.add_comment(" Hello World ");
    assert_eq!("<!-- Hello World -->", write_element(&doc));
}

#[test]
fn single_cdata() {
    let mut doc = Element::new("#document");
    doc.add_cdata(" Hello World ");
    assert_eq!("<![CDATA[ Hello World ]]>", write_element(&doc));
}

#[test]
fn single_raw_element() {
    let mut doc = Element::new("#document");
    let mut e = Element::new("script");
    e.nodes.push(Node::Raw(Raw::new(" ><&\"'\u{a0}äöü ")));
    doc.nodes.push(Node::Element(e));
    assert_eq!("<script> ><&\"'\u{a0}äöü </script>", write_element(&doc));
}