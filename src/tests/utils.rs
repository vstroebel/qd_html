
use crate::utils::*;

#[test]
fn autoclose_element() {
    assert!(is_autoclose_element("br"));
    assert!(!is_autoclose_element("div"));
}

#[test]
fn raw_element() {
    assert!(is_raw_element("script"));
    assert!(is_raw_element("style"));
    assert!(!is_raw_element("div"));
}

#[test]
fn html_whitespace() {
    assert!(is_html_whitespace(' '));
    assert!(is_html_whitespace('\n'));
    assert!(is_html_whitespace('\r'));
    assert!(is_html_whitespace('\t'));
    assert!(!is_html_whitespace('\u{a0}'));
}

#[test]
fn empty() {
    assert_eq!(decode_entity(""), None);
}

#[test]
fn unknown() {
    assert_eq!(decode_entity("Not an entity"), None);
}

#[test]
fn named_entity() {
    assert_eq!(decode_entity("euro"), Some('€'));
}

#[test]
fn hex_entity() {
    assert_eq!(decode_entity("#x1F4A9"), Some('💩'));
}

#[test]
fn hex_entity2() {
    assert_eq!(decode_entity("#X1F4A9"), Some('💩'));
}

#[test]
fn bad_hex() {
    assert_eq!(decode_entity("#X"), None);
}

#[test]
fn bad_hex2() {
    assert_eq!(decode_entity("#XZ1F4A9"), None);
}

#[test]
fn bad_hex3() {
    assert_eq!(decode_entity("#X1F4A9FFFFF"), None);
}


#[test]
fn dec_entity() {
    assert_eq!(decode_entity("#128169"), Some('💩'));
}

#[test]
fn bad_dec() {
    assert_eq!(decode_entity("#"), None);
}

#[test]
fn bad_dec2() {
    assert_eq!(decode_entity("#Z111"), None);
}

#[test]
fn bad_dec3() {
    assert_eq!(decode_entity("#99999999999999999999"), None);
}
