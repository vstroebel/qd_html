# qd_html

A simple rust library for working with HTML.

![Tests](https://github.com/vstroebel/qd_html/workflows/Tests/badge.svg)

Feature:
 - parser with a SAX alike interface
 - writer
 - minimalistic DOM implementation
 - some utilities
 - installable app to clean up HTML files
  
**NOTE**: This is **NOT** a fully standard compliant HTML parser.
If you need this [html5ever](https://crates.io/crates/html5ever) might be a better solution.

## Getting Started

Add the following dependency to your Cargo manifest.

```toml
[dependencies]
qd_html = "0.1.0"
```

## Example

```rust
use qd_html::parser::parse_to_dom;
use qd_html::writer::write;

fn main() {

    let html = "<html><head><title>EXAMPLE</title></body></html>";

    let document = parse_to_dom(html);

    let html = write(&document);

    println!("{}", html);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
