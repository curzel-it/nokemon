use std::collections::HashMap;

use crate::utils::vector_utils::make_lookup;

use super::species_model::Species;
use super::species_parser::SpeciesParser;

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

    pub fn number_of_available_species(&self) -> usize {
        return self.species_by_id.len();
    }

    pub fn available_species(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.species_by_id.keys().cloned().collect();
        ids.sort();
        ids
    }

    pub fn species(&self, species_id: String) -> Species {
        let species = self.species_by_id.get(&species_id);
        return match species {
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

        let expected_number_of_species = species_paths.len();
        assert_eq!(repo.number_of_available_species(), expected_number_of_species);

        assert_eq!(repo.species("ape".to_owned()).id, "ape");
        assert_eq!(repo.species("cybertruck".to_owned()).id, "cybertruck");
        assert_eq!(repo.species("tower".to_owned()).id, "tower");
        assert_eq!(repo.species("towerdart".to_owned()).id, "towerdart");
    }
}
