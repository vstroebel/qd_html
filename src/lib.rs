// Copyright 2020 Volker Ströbel
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A simple rust library for working with HTML.
//!
//! # Examples
//!
//! ## Parse a HTML file into a document tree
//!
//! ```rust
//! use qd_html::parser::parse_to_dom;
//!
//! let html = "<html><head></head><body></body></html>";
//!
//! let document = parse_to_dom(html);
//! ```
//!
//! ## Create a simple document and write it to html
//!
//! ```rust
//! use qd_html::dom::{Document, Element};
//! use qd_html::writer::write;
//!
//! let mut doc = Document::new();
//!
//! let mut root = Element::new("html");
//! root.add_element(Element::new("head"));
//! root.add_element(Element::new("body"));
//!
//! doc.element.add_element(root);
//!
//! let html = write(&doc);
//!
//! assert_eq!(html, "<!DOCTYPE html><html><head></head><body></body></html>");
//!
//! ```

#[macro_use]
extern crate lazy_static;

mod reader;

pub mod utils;
pub mod dom;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests;