use std::collections::HashMap;

use crate::utils::grouping_utils::make_lookup;

use super::species_model::Species;
use super::species_parser::SpeciesParser;

#[derive(Debug)]
pub struct SpeciesRepository {
    parser: SpeciesParser,
    species_by_id: HashMap<String, Species>,
}

impl SpeciesRepository {
    pub fn new(parser: SpeciesParser) -> Self {
        SpeciesRepository {
            parser,
            species_by_id: HashMap::new(),
        }
    }

    pub fn setup(&mut self, paths: &Vec<String>) {
        let all_species: Vec<Species> = paths.iter().filter_map(|path| self.parser.parse_from_file(path)).collect();
        let lookup = make_lookup(&all_species, |species: &Species| species.id.clone());
        self.species_by_id = lookup;
    }

    pub fn species(&self, species_id: &String) -> Species {
        let species = self.species_by_id.get(species_id);
        match species {
            Some(species) => species.clone(),
            None => Species::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::SPECIES_PATH;
    use crate::utils::file_utils::list_files;

    #[test]
    fn can_be_setup_and_load_all_species() {
        let parser = SpeciesParser::new();
        let mut repo = SpeciesRepository::new(parser);

        let species_paths = list_files(SPECIES_PATH, "json");

        repo.setup(&species_paths);

        assert_eq!(repo.species(&"red".to_owned()).id, "red");
        assert_eq!(repo.species(&"blue".to_owned()).id, "blue");
        assert_eq!(repo.species(&"white".to_owned()).id, "white");
        assert_eq!(repo.species(&"tower".to_owned()).id, "tower");
        assert_eq!(repo.species(&"towerdart".to_owned()).id, "towerdart");
        assert_eq!(repo.species(&"cybertruck".to_owned()).id, "cybertruck");
    }
}
