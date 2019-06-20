extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use regex::Regex;

fn main() {
    println!("Start converting ...");

    let css_path_string =
        String::from(env!("CARGO_MANIFEST_DIR").to_owned() + "/resources/nerd-fonts-generated.css");
    let css_path = Path::new(&css_path_string);
    let css_file = File::open(&css_path);
    let mut css_file = match css_file {
        Ok(file) => file,
        Err(error) => panic!("Couldn't open file {}: {}", css_path.display(), error),
    };
    let mut css_contents = String::new();

    match css_file.read_to_string(&mut css_contents) {
        Ok(_) => {}
        Err(error) => panic!("Couldn't read file {}: {}", css_path.display(), error),
    }

    let re = Regex::new(r#"(nf-\S+):.+\n.+content.+"\\(.+)""#).unwrap();

    for caps in re.captures_iter(&css_contents) {
        println!(
            "{} {}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str()
        )
    }
}