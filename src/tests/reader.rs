
use crate::reader::Reader;

fn new_from_str(s: &str) -> Reader {
    Reader::new(s)
}

#[test]
fn empty() {
    let mut r = new_from_str("");
    assert_eq!(r.get_until(false, &[]), "".to_owned());
}

#[test]
fn read_until_endchar() {
    let mut r = new_from_str("some! ");
    assert_eq!(r.get_until(true, &['!']), "some".to_owned());
}

#[test]
fn read_until_ws() {
    let mut r = new_from_str("some !");
    assert_eq!(r.get_until(true, &['!']), "some".to_owned());
}

#[test]
fn read_until_with_ws() {
    let mut r = new_from_str(" some!");
    assert_eq!(r.get_until(false, &['!']), " some".to_owned());
}

#[test]
fn skip_ws() {
    let mut r = new_from_str(" some!");
    r.skip_whitespace();
    assert_eq!(r.get_until(false, &['!']), "some".to_owned());
}

#[test]
fn iter() {
    let mut r = new_from_str("some! ");

    assert_eq!(r.next_char(), Some('s'));
    assert_eq!(r.next_char(), Some('o'));
    assert_eq!(r.next_char(), Some('m'));
    assert_eq!(r.next_char(), Some('e'));
    assert_eq!(r.next_char(), Some('!'));
    assert_eq!(r.next_char(), Some(' '));
    assert_eq!(r.next_char(), None);
}

#[test]
fn pushback() {
    let mut r = new_from_str("abc");

    assert_eq!(r.next_char(), Some('a'));
    assert_eq!(r.next_char(), Some('b'));

    r.push_back('b');

    assert_eq!(r.next_char(), Some('b'));
    assert_eq!(r.next_char(), Some('c'));
    assert_eq!(r.next_char(), None);
}

#[test]
fn entity() {
    let mut r = new_from_str("a&amp;b<");
    assert_eq!(r.read_text(&['<']), "a&b");
}

#[test]
fn badentity() {
    let mut r = new_from_str("a&");
    assert_eq!(r.read_text(&['<']), "a&");
}

#[test]
fn badentity2() {
    let mut r = new_from_str("a&;");
    assert_eq!(r.read_text(&['<']), "a&;");
}

#[test]
fn badhexentity() {
    let mut r = new_from_str("a&#;");
    assert_eq!(r.read_text(&['<']), "a&#;");
}

#[test]
fn read_raw_count() {
    let mut r = new_from_str("123456789");
    assert_eq!(r.read_raw_count(6), "123456");
}

#[test]
fn read_raw_count_insufficient_input() {
    let mut r = new_from_str("123");
    assert_eq!(r.read_raw_count(6), "123");
}

#[test]
fn ignore_if_next_true() {
    let mut r = new_from_str("some! ");

    assert_eq!(r.next_char(), Some('s'));

    assert!(r.ignore_if_next('o'));

    assert_eq!(r.next_char(), Some('m'));
    assert_eq!(r.next_char(), Some('e'));
    assert_eq!(r.next_char(), Some('!'));
    assert_eq!(r.next_char(), Some(' '));
    assert_eq!(r.next_char(), None);
}

#[test]
fn ignore_if_next_false() {
    let mut r = new_from_str("some! ");

    assert_eq!(r.next_char(), Some('s'));

    assert!(!r.ignore_if_next('s'));

    assert_eq!(r.next_char(), Some('o'));
    assert_eq!(r.next_char(), Some('m'));
    assert_eq!(r.next_char(), Some('e'));
    assert_eq!(r.next_char(), Some('!'));
    assert_eq!(r.next_char(), Some(' '));
    assert_eq!(r.next_char(), None);
}

#[test]
fn test_valid_chars() {
    //Not testing all possible u32 values because this would be too slow...
    for i in 0..10_000_000{
        if let Some(ch) = ::std::char::from_u32(i){
            let s = ch.to_string();
            let mut r = new_from_str(&s);
            assert_eq!(r.next_char(), Some(ch));
            assert_eq!(r.next_char(), None);
        }
    }
}