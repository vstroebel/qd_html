
use std::collections::HashSet;
use std::collections::HashMap;
use std::char::from_u32;

include!("entities_map.rs");

lazy_static! {
  static ref ENTITIES: HashMap<&'static str, char> = create_entity_map();
}

lazy_static! {
  static ref AUTOCLOSE_ELEMENTS: HashSet<&'static str> = {
    let mut set = HashSet::new();
    set.insert("meta");
    set.insert("link");
    set.insert("base");
    set.insert("br");
    set.insert("wbr");
    set.insert("hr");
    set.insert("img");
    set.insert("input");
    set.insert("frame");
    set.insert("embed");
    set.insert("col");
    set.insert("command");
    set.insert("source");
    set.insert("device");
    set.insert("keygen");
    set
  };
}

///Test a tag name referes to a tag that doesn't have child elements i.e. &lt;br&gt;
pub fn is_autoclose_element(name: &str) -> bool {
    AUTOCLOSE_ELEMENTS.contains(name)
}

pub fn is_raw_element(name: &str) -> bool {
    name == "script" || name == "style"
}

pub fn is_html_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\n' || ch == '\r' || ch == '\t'
}

pub fn decode_entity(entity: &str) -> Option<char> {

    if entity.starts_with('#') {
        if entity.starts_with("#x") || entity.starts_with("#X") {
            return match u32::from_str_radix(&entity[2..], 16) {
                Ok(cp) => from_u32(cp),
                Err(_) => None,
            };
        } else {
            return match u32::from_str_radix(&entity[1..], 10) {
                Ok(cp) => from_u32(cp),
                Err(_) => None,
            };
        }
    }

    ENTITIES.get(entity).cloned()
}

///Returns a cleaned up version of the input HTML
///
/// This parses the html and write the resulting document without modifications.
/// The result is not guaranteed to be a valid HTML but simple errors like missing close tags should be removed.

/// # Examples
///
/// ```rust
/// use qd_html::utils::cleanup;
///
/// let input_html = "<!DOCTYPE html><html><body><h1>test</body></html>";
/// let output_html = cleanup(input_html);
/// assert_eq!(output_html, "<!DOCTYPE html><html><body><h1>test</h1></body></html>");
/// ```
pub fn cleanup(html: &str) -> String {
    crate::writer::write(&crate::parser::parse_to_dom(html))
}
