
use crate::parser::parse_to_dom;
use crate::dom::*;

#[test]
fn empty() {
    let doc = parse_to_dom("");
    assert_eq!(doc.doctype, None);
    let e = doc.element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 0);
}

#[test]
fn doctype() {
    let doc = parse_to_dom("<!DOCTYPE html>");
    assert_eq!(doc.doctype, Some("html".to_owned()));
    let e = doc.element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 0);
}

#[test]
fn bad_doctype() {
    let doc = parse_to_dom("<!DOCTYPe html>Some");
    assert_eq!(doc.doctype, None);
    let e = doc.element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 2);

    if let &Node::Text(ref text) = &e.nodes[0] {
        assert_eq!(text.content, "<!DOCTYPe html>");
    } else {
        panic!("Node of wrong type");
    }

    if let &Node::Text(ref text) = &e.nodes[1] {
        assert_eq!(text.content, "Some");
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn text_only() {
    let e = parse_to_dom("HELLO World!").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Text(ref text) = &e.nodes[0] {
        assert_eq!(text.content, "HELLO World!");
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn text_only_with_entities() {
    let e = parse_to_dom("HELL&Ouml; W&ouml;rld!").element;

    if let &Node::Text(ref text) = &e.nodes[0] {
        assert_eq!(text.content, "HELLÖ Wörld!");
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn text_only_with_bad_entities() {
    let e = parse_to_dom("HELL&ZZZ; W&#xZZZ;!").element;

    if let &Node::Text(ref text) = &e.nodes[0] {
        assert_eq!(text.content, "HELL&ZZZ; W&#xZZZ;!");
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn text_only_with_unclosed_entities() {
    let e = parse_to_dom("HELL&Ouml W&oumlrld!").element;

    if let &Node::Text(ref text) = &e.nodes[0] {
        assert_eq!(text.content, "HELL&Ouml W&oumlrld!");
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_element() {
    let e = parse_to_dom("<test></test>").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test");
        assert_eq!(e.attributes.len(), 0);
        assert_eq!(e.nodes.len(), 0);
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_element_without_content() {
    let e = parse_to_dom("<test />").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test");
        assert_eq!(e.attributes.len(), 0);
        assert_eq!(e.nodes.len(), 0);
        assert!(e.autoclosed);
    } else {
        panic!("Node of wrong type");
    }
}


#[test]
fn single_element_with_dublicate_attribute() {
    let e = parse_to_dom("<test attr1=\"a1\" attr1='a2'></test>").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test");
        assert_eq!(e.attributes.len(), 1, "Bad attributes:{:?}", e);
        assert_eq!(e.attributes.get_value("attr1"), Some("a2"));
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_element_with_dublicate_bool_attribute() {
    let e = parse_to_dom("<test attr1=\"a1\" attr1></test>").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test");
        assert_eq!(e.attributes.len(), 1, "Bad attributes:{:?}", e);
        assert!(e.attributes.contains("attr1"));
        assert_eq!(e.attributes.get_value("attr1"), None);
    } else {
        panic!("Node of wrong type");
    }
}


#[test]
fn single_element_with_attributes() {
    let e = parse_to_dom("<test attr1=\"a1\" attr2='a2' attr3=a3 attr4></test>").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test");
        assert_eq!(e.attributes.len(), 4, "Bad attributes:{:?}", e);

    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_element_with_qm_in_attribute_name() {
    //? in attribute name is allowed in html5 and needed by the template system to mark boolean attributes
    let e = parse_to_dom("<test attr1?=\"a1\"></test>").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test");
        assert_eq!(e.attributes.len(), 1, "Bad attributes:{:?}", e);
        assert!(e.attributes.contains("attr1?"));
        assert_eq!(e.attributes.get_value("attr1?"), Some("a1"));
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_element_ws() {
    let e = parse_to_dom("< test ></ test >").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test");
        assert_eq!(e.attributes.len(), 0);
        assert_eq!(e.nodes.len(), 0);
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn multiple_elements() {
    let e = parse_to_dom("<foo><bar>Hello</bar><maman>World</maman></foo>").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "foo");

        if let &Node::Element(ref e) = &e.nodes[0] {
            assert_eq!(e.name, "bar");
            if let &Node::Text(ref t) = &e.nodes[0] {
                assert_eq!(t.content, "Hello");
            } else {
                panic!("Node of wrong type");
            }
        } else {
            panic!("Node of wrong type");
        }

        if let &Node::Element(ref e) = &e.nodes[1] {
            assert_eq!(e.name, "maman");
            if let &Node::Text(ref t) = &e.nodes[0] {
                assert_eq!(t.content, "World");
            } else {
                panic!("Node of wrong type");
            }
        } else {
            panic!("Node of wrong type");
        }


    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn unclosed_elements() {
    let e = parse_to_dom("<test><foo><bar>").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test");
        assert_eq!(e.attributes.len(), 0);
        assert_eq!(e.nodes.len(), 1);
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_comment() {
    let e = parse_to_dom("<!-- Hello World -->").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Comment(ref e) = &e.nodes[0] {
        assert_eq!(e.content, " Hello World ");
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_script() {
    let e = parse_to_dom("<script> \"<test></test>\" </script>").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "script");
        assert_eq!(e.nodes.len(), 1);
        if let &Node::Raw(ref raw) = &e.nodes[0] {
            assert_eq!(raw.content, " \"<test></test>\" ");
        } else {
            panic!("Node of wrong type");
        }
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_style_mixedcase() {
    let e = parse_to_dom("<STyle> \"<test></test>\" </STyle>").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "style");
        assert_eq!(e.nodes.len(), 1);
        if let &Node::Raw(ref raw) = &e.nodes[0] {
            assert_eq!(raw.content, " \"<test></test>\" ");
        } else {
            panic!("Node of wrong type");
        }
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_element_mixing_case() {
    let e = parse_to_dom("<foo><teST></TEst><bar></bar></foo>").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "foo");
        assert_eq!(e.nodes.len(), 2);

        if let &Node::Element(ref e) = &e.nodes[0] {
            assert_eq!(e.name, "test");
            assert_eq!(e.nodes.len(), 0);
        } else {
            panic!("Node of wrong type");
        }

        if let &Node::Element(ref e) = &e.nodes[1] {
            assert_eq!(e.name, "bar");
            assert_eq!(e.nodes.len(), 0);
        } else {
            panic!("Node of wrong type");
        }

    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_cdata() {
    let e = parse_to_dom("<![CDATA[ Hello World ]]>").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::CData(ref e) = &e.nodes[0] {
        assert_eq!(e.content, " Hello World ");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn bad_cdata() {
    let e = parse_to_dom("<![CDATa[ Hello World ]]>").element;

    assert_eq!(e.name, "#document");
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Text(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "<![CDATa[ Hello World ]]>");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn not_realy_a_doctype() {
    let e = parse_to_dom("<!DOCTYPl foo>").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Text(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "<!DOCTYPl foo>");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn unknown_bang_stuff() {
    let e = parse_to_dom("<!-foo >").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Text(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "<!-foo >");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn unknown_short_bang_stuff_short1() {
    let e = parse_to_dom("<!><test></test>").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 2);

    if let &Node::Text(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "<!>");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }

    if let &Node::Element(ref e) = &e.nodes[1] {
        assert_eq!(e.name, "test");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn unknown_short_bang_stuff_short2() {
    let e = parse_to_dom("<!a><test>").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 2);

    if let &Node::Text(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "<!a>");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }

    if let &Node::Element(ref e) = &e.nodes[1] {
        assert_eq!(e.name, "test");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn parse_processing_instruction() {
    let e = parse_to_dom("<?what ever ?>").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::ProcessingInstruction(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "what ever ");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn unclosed_lt() {
    let e = parse_to_dom("<").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Text(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "<");
    } else {
        panic!("Node of wrong type:{:?}", e);
    }
}

#[test]
fn single_element_unclosed_start_element() {
    let e = parse_to_dom("<test attr1=\"a1\" ").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test");
        assert_eq!(e.attributes.len(), 1, "Bad attributes:{:?}", e);
        assert_eq!(e.attributes.get_value("attr1"), Some("a1"));
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn single_element_unclosed_attribute() {
    let e = parse_to_dom("<test attr1=").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Element(ref e) = &e.nodes[0] {
        assert_eq!(e.name, "test");
        assert_eq!(e.attributes.len(), 1, "Bad attributes:{:?}", e);
        assert!(e.attributes.contains("attr1"));
        assert_eq!(e.attributes.get_value("attr1"), None);
    } else {
        panic!("Node of wrong type");
    }
}

#[test]
fn processing_instruction_with_multicode_point_grapheme() {
    //Found with fuzzer
    let e = parse_to_dom("<!Ύ>").element;

    assert_eq!(e.name, "#document", "Bad document:{:?}", e);
    assert_eq!(e.attributes.len(), 0);
    assert_eq!(e.nodes.len(), 1);

    if let &Node::Text(ref e) = &e.nodes[0] {
        assert_eq!(e.content, "<!Ύ>");
    } else {
        panic!("Node of wrong type");
    }
}