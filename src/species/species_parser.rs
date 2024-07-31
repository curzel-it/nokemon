use std::fs;
use std::io::Read;

use super::species_model::Species;

#[derive(Debug)]
pub struct SpeciesParser;

impl SpeciesParser {
    pub fn new() -> Self {
        SpeciesParser {}
    }

    pub fn parse_from_file(&self, file_path: &String) -> Option<Species> {
        let mut file = match fs::File::open(file_path) {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut buffer = String::new();
        if let Err(_) = file.read_to_string(&mut buffer) {
            return None;
        }
        self.parse(&buffer)
    }

    pub fn parse(&self, json_string: &str) -> Option<Species> {
        match serde_json::from_str(json_string) {
            Ok(species) => Some(species),
            Err(err) => {
                eprintln!("Failed to parse species: {}", err);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::constants::SPECIES_PATH;

    use super::*;

    #[test]
    fn returns_null_with_empty_string() {
        let parser = SpeciesParser;
        let result = parser.parse("");
        assert!(result.is_none());
    }

    #[test]
    fn returns_null_with_malformed_json() {
        let parser = SpeciesParser;
        let malformed_json = r#"{id:"Tiger", "speed":35.5, "scale":1.2"#;
        let result = parser.parse(malformed_json);
        assert!(result.is_none());
    }

    #[test]
    fn can_parse_species_from_file_path_without_scale() {
        let parser = SpeciesParser;
        let path = Path::new(SPECIES_PATH).join("tower.json");
        let path_string = path.to_str().unwrap().to_owned();
        let result = parser.parse_from_file(&path_string).unwrap();
        assert_eq!(result.id, "tower");
        assert_eq!(result.speed, 0.0);
        assert_eq!(result.scale, 1.0);
        assert_eq!(result.bullets_per_minute, 120.0);
    }

    #[test]
    fn can_parse_species_from_file_path_with_scale() {
        let parser = SpeciesParser;
        let path = Path::new(SPECIES_PATH).join("cybertruck.json");
        let path_string = path.to_str().unwrap().to_owned();
        let result = parser.parse_from_file(&path_string).unwrap();
        assert_eq!(result.id, "cybertruck");
        assert_eq!(result.speed, 1.7);
        assert_eq!(result.scale, 2.0);
        assert_eq!(result.bullets_per_minute, 1.0);
    }
}
