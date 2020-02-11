//! [Nerd Fonts](https://github.com/ryanoasis/nerd-fonts) in rust.
//!
//! # How to use

//! In your `Cargo.toml`, add:

//! ```toml
//! [dependencies]
//! nerd_fonts = "0.1"
//! ```

//! Then import it:

//! ```rust
//! extern crate nerd_fonts;
//! ```

//! Use `NerdFonts::load` to load:

//! ```rust
//! use nerd_fonts::NerdFonts;

//! let nf = NerdFonts {
//!     nf: NerdFonts::load(),
//! };

//! let nf_custom_c = nf.get("custom-c").unwrap(); // '\u{e61e}'
//! ```

extern crate yaml_rust;

use std::char;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use yaml_rust::{Yaml, YamlLoader};

/// NerdFonts includes nf field which stores the whole nerd fonts Yaml.
pub struct NerdFonts {
    pub nf: Yaml,
}

impl NerdFonts {
    /// Load nerd fonts.
    pub fn load() -> Yaml {
        let nf_yaml_path_string = String::from(
            env!("CARGO_MANIFEST_DIR").to_owned() + "/resources/nerd-fonts-generated.yaml",
        );
        let nf_yaml_path = Path::new(&nf_yaml_path_string);
        let mut nf_yaml_file = match File::open(&nf_yaml_path) {
            Ok(file) => file,
            Err(error) => panic!("Couldn't open file {}: {}", nf_yaml_path.display(), error),
        };
        let mut nf_yaml_contents = String::new();

        match nf_yaml_file.read_to_string(&mut nf_yaml_contents) {
            Ok(_) => {}
            Err(error) => panic!("Couldn't read file {}: {}", nf_yaml_path.display(), error),
        }

        let nf = YamlLoader::load_from_str(&nf_yaml_contents)
            .unwrap()
            .pop()
            .unwrap();

        nf
    }

    /// get the single font by name.
    pub fn get(&self, name: &str) -> Option<char> {
        if let Some(f) = self.nf[name].as_str() {
            u32::from_str_radix(f, 16).ok().and_then(char::from_u32)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NerdFonts;

    #[test]
    fn nerd_fonts_works() {
        let nf = NerdFonts {
            nf: NerdFonts::load(),
        };

        assert_eq!(nf.get("custom-c").unwrap(), '\u{e61e}');
        assert_eq!(nf.get("weather-windy").unwrap(), '\u{e31e}');
        assert_eq!(nf.get("fae-cheese").unwrap(), '\u{e264}');
        assert_eq!(nf.get("abc"), None);
    }
}
