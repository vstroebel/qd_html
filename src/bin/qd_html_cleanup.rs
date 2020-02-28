use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use qd_html::utils::cleanup;

fn main() {
    let path = match env::args().nth(2) {
        Some(path) => path,
        None => {
            print!("Usage: qd_html_cleanup <FILE>");
            return;
        }
    };

    let path = Path::new(&path);

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening file {:?} -> {:?}", path, err),
    };

    let mut content = String::new();

    file.read_to_string(&mut content).expect("Reading file");

    println!("{}", cleanup(&content));
}
