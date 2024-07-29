use std::collections::HashMap;

use regex::Regex;

use super::sprite_set::SpriteSet;

#[derive(Debug, Clone, PartialEq)]
struct SpriteFrame {
    path: String,
    species: String,
    animation: String,
    index: u32
}

impl SpriteFrame {
    fn new(path: &str, species: &str, animation: &str, index: u32) -> Self {
        SpriteFrame {
            path: path.to_owned(),
            species: species.to_owned(),
            animation: animation.to_owned(),
            index: index
        }
    }
}

#[derive(Debug)]
pub struct SpriteSetBuilder;

impl SpriteSetBuilder {
    pub fn new() -> Self {
        SpriteSetBuilder {}
    }

    pub fn sprite_sets(&self, paths: &[String]) -> HashMap<String, SpriteSet> {
        let frames = self.sprite_frames_from_paths(paths);
        let frames_by_species = self.aggregate_frames_by_species(&frames);

        let mut sets_by_species = HashMap::new();
        for (species, frames) in frames_by_species {
            if let Some(set) = self.sprite_set(&frames) {
                sets_by_species.insert(species, set);
            }
        }
        sets_by_species
    }

    fn sprite_frames_from_paths(&self, paths: &[String]) -> Vec<SpriteFrame> {
        let mut frames = Vec::new();
        for path in paths {
            if let Some(frame) = self.sprite_frame_from_path(path) {
                frames.push(frame);
            }
        }
        frames.sort_by(|a, b| {
            a.species.cmp(&b.species)
                .then(a.animation.cmp(&b.animation))
                .then(a.index.cmp(&b.index))
        });
        frames
    }

    fn sprite_frame_from_path(&self, path: &str) -> Option<SpriteFrame> {        
        let re = Regex::new(r"^(.+?)_([a-zA-Z]+)-([0-9]+)$").unwrap();
        let file_name = std::path::Path::new(path).file_stem().unwrap().to_str().unwrap();

        if let Some(caps) = re.captures(file_name) {
            let species = caps.get(1).unwrap().as_str().to_string();
            let animation_name = caps.get(2).unwrap().as_str().to_string();
            let index = caps.get(3).unwrap().as_str().parse::<u32>().unwrap_or(0);
            let frame = SpriteFrame::new(path, &species, &animation_name, index);
            return Some(frame);
        }
        None
    }

    fn aggregate_frames_by_species(&self, frames: &[SpriteFrame]) -> HashMap<String, Vec<SpriteFrame>> {
        let mut map = HashMap::new();
        for frame in frames {
            map.entry(frame.species.clone())
                .or_insert_with(Vec::new)
                .push(frame.clone());
        }
        map
    }

    fn sprite_set(&self, frames: &[SpriteFrame]) -> Option<SpriteSet> {
        let mut frames_by_animation = HashMap::new();
        for frame in frames {
            frames_by_animation
                .entry(frame.animation.clone())
                .or_insert_with(Vec::new)
                .push(frame.path.clone());
        }
        Some(SpriteSet::new(frames_by_animation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_frame_from_path() {
        let builder = SpriteSetBuilder;

        assert_eq!(
            builder.sprite_frame_from_path("/ape_eat-0.png"),
            Some(SpriteFrame::new("/ape_eat-0.png", "ape", "eat", 0))
        );
        assert_eq!(
            builder.sprite_frame_from_path("/ape_chef_eat-0.png"),
            Some(SpriteFrame::new("/ape_chef_eat-0.png", "ape_chef", "eat", 0))
        );
        assert_eq!(
            builder.sprite_frame_from_path("/ape_eat-123.png"),
            Some(SpriteFrame::new("/ape_eat-123.png", "ape", "eat", 123))
        );
        assert_eq!(
            builder.sprite_frame_from_path("/ape_chef_eat-123.png"),
            Some(SpriteFrame::new("/ape_chef_eat-123.png", "ape_chef", "eat", 123))
        );
    }

    #[test]
    fn sprite_frames_from_paths() {
        let builder = SpriteSetBuilder;

        let paths = vec![
            "/ape_chef_eat-1.png".to_string(),
            "/ape_chef_eat-2.png".to_string(),
            "/ape_chef_eat-3.png".to_string(),
            "/ape_chef_eat-4.png".to_string(),
            "/ape_chef_eat-invalid.png".to_string(),
            "/invalid.png".to_string(),
        ];
        assert_eq!(builder.sprite_frames_from_paths(&paths).len(), 4);
    }

    #[test]
    fn can_aggregate_frames_by_species() {
        let builder = SpriteSetBuilder;

        let frames = vec![
            SpriteFrame::new("a_b-0", "a", "b", 0),
            SpriteFrame::new("a_b-1", "a", "b", 1),
            SpriteFrame::new("a_b-2", "a", "b", 2),
            SpriteFrame::new("c_d-0", "c", "d", 0),
            SpriteFrame::new("c_d-1", "c", "d", 1),
            SpriteFrame::new("e_f-0", "e", "f", 0),
            SpriteFrame::new("e_f-1", "e", "f", 1),
        ];

        let result = builder.aggregate_frames_by_species(&frames);

        let frames_a = vec![
            SpriteFrame::new("a_b-0", "a", "b", 0),
            SpriteFrame::new("a_b-1", "a", "b", 1),
            SpriteFrame::new("a_b-2", "a", "b", 2),
        ];
        assert_eq!(result["a"], frames_a);

        let frames_c = vec![
            SpriteFrame::new("c_d-0", "c", "d", 0),
            SpriteFrame::new("c_d-1", "c", "d", 1),
        ];
        assert_eq!(result["c"], frames_c);

        let frames_e = vec![
            SpriteFrame::new("e_f-0", "e", "f", 0),
            SpriteFrame::new("e_f-1", "e", "f", 1),
        ];
        assert_eq!(result["e"], frames_e);
    }
}
