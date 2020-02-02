// Copyright 2020 Volker Ströbel
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate lazy_static;

mod reader;
mod utils;

pub mod dom;
pub mod parser;

#[cfg(test)]
mod tests;