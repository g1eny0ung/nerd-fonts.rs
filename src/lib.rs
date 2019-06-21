extern crate yaml_rust;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use yaml_rust::{Yaml, YamlLoader};

#[allow(dead_code)]
pub struct NerdFonts {
    nf: Yaml,
}

#[allow(dead_code)]
impl NerdFonts {
    fn load() -> Yaml {
        let nf_yaml_path_string = String::from(
            env!("CARGO_MANIFEST_DIR").to_owned() + "/resources/nerd-fonts-generated.yaml",
        );
        let nf_yaml_path = Path::new(&nf_yaml_path_string);
        let nf_yaml_file = File::open(&nf_yaml_path);
        let mut nf_yaml_file = match nf_yaml_file {
            Ok(file) => file,
            Err(error) => panic!("Couldn't open file {}: {}", nf_yaml_path.display(), error),
        };
        let mut nf_yaml_contents = String::new();

        match nf_yaml_file.read_to_string(&mut nf_yaml_contents) {
            Ok(_) => {}
            Err(error) => panic!("Couldn't read file {}: {}", nf_yaml_path.display(), error),
        }

        let yaml = YamlLoader::load_from_str(&nf_yaml_contents).unwrap();
        let nf = &yaml[0];

        nf.clone()
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.nf[key].as_str()
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

        assert_eq!(nf.get("custom-c").unwrap(), "e61e");
        assert_eq!(nf.get("weather-windy").unwrap(), "e31e");
    }
}
