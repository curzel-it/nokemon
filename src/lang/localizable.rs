use crate::constants::DEFAULT_LANG;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use lazy_static::lazy_static;
use crate::constants::LOCALIZED_STRINGS_PATH;

pub trait LocalizableText {
    fn localized(&self) -> String; 
}

impl LocalizableText for String {
    fn localized(&self) -> String {
        if let Some(strings) = LOCALIZED_STRINGS.get(DEFAULT_LANG) {
            if let Some(localized_string) = strings.get(self) {
                return localized_string.clone();
            }
        }
        self.clone()
    }
}

impl LocalizableText for &str {
    fn localized(&self) -> String {
        self.to_string().localized()
    }
}

lazy_static! {
    pub static ref LOCALIZED_STRINGS: HashMap<String, HashMap<String, String>> = load_localized_strings();
}

fn load_localized_strings() -> HashMap<String, HashMap<String, String>> {
    let mut localized_strings = HashMap::new();    
    let paths = fs::read_dir(LOCALIZED_STRINGS_PATH)
        .expect("Failed to read localized strings directory")
        .flatten()
        .map(|p| p.path());

    for file_path in paths {        
        if file_path.extension() == Some(std::ffi::OsStr::new("strings")) {
            if let Some(locale) = file_path.file_stem().and_then(|os_str| os_str.to_str()) {
                let strings = load_strings_from_file(&file_path);
                localized_strings.insert(locale.to_string(), strings);
            }
        }
    }
    localized_strings
}

fn load_strings_from_file(file_path: &Path) -> HashMap<String, String> {
    let mut map = HashMap::new();
    
    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read localization file: {:?}", file_path));

    for line in content.lines() {
        if let Some((key, value)) = parse_line(line) {
            map.insert(key, value);
        }
    }
    map
}

fn parse_line(line: &str) -> Option<(String, String)> {
    let trimmed_line = line.trim();

    if trimmed_line.starts_with("//") || trimmed_line.is_empty() {
        return None;
    }

    if let Some(equal_sign_pos) = trimmed_line.find('=') {
        let key = trimmed_line[..equal_sign_pos].trim().trim_matches('"').to_string();
        let value = trimmed_line[equal_sign_pos + 1..]
            .trim()
            .trim_matches('"')
            .trim_end_matches("\";")
            .replace(r"\n", "\n")
            .replace(r#"\""#, r#"""#)
            .replace(r"\\", "\\")
            .replace(r"\t", "\t")
            .replace(r"\r", "\r");

        Some((key, value))
    } else {
        None
    }
}