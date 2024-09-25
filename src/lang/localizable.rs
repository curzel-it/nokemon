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
    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read localization file: {:?}", file_path));
    parse_strings_content(&content)
}

fn parse_strings_content(content: &str) -> HashMap<String, String> {
    let mut strings_map = HashMap::new();
    let mut pos = 0;
    let content_chars: Vec<char> = content.chars().collect();
    let len = content.len().min(content_chars.len());

    while pos < len {
        skip_whitespace(&content_chars, &mut pos);

        if pos >= len {
            break;
        }

        if content_chars[pos] == '"' {
            // Parse the key string
            let key = parse_string(&content_chars, &mut pos);
            skip_whitespace(&content_chars, &mut pos);

            // Expect '='
            if pos >= len || content_chars[pos] != '=' {
                panic!("Expected '=' after key at position {}", pos);
            }
            pos += 1; // Skip '='

            skip_whitespace(&content_chars, &mut pos);

            // Parse the value
            let value = if content_chars[pos] == '"' {
                if pos + 2 < len && content_chars[pos + 1] == '"' && content_chars[pos + 2] == '"' {
                    // Multiline string
                    parse_multiline_string(&content_chars, &mut pos)
                } else {
                    // Single line string
                    parse_string(&content_chars, &mut pos)
                }
            } else {
                panic!("Expected '\"' at position {}", pos);
            };

            strings_map.insert(key, value);
        } else {
            // Unexpected character
            panic!("Expected '\"' at position {}", pos);
        }
    }

    strings_map
}

fn skip_whitespace(chars: &[char], pos: &mut usize) {
    while *pos < chars.len() && chars[*pos].is_whitespace() {
        *pos += 1;
    }
}

fn parse_string(chars: &[char], pos: &mut usize) -> String {
    if chars[*pos] != '"' {
        panic!("Expected '\"' at position {}", pos);
    }
    *pos += 1; // Skip opening '"'

    let mut result = String::new();
    while *pos < chars.len() {
        let c = chars[*pos];
        if c == '"' {
            *pos += 1; // Skip closing '"'
            return result;
        } else if c == '\\' {
            // Handle escaped characters
            *pos += 1;
            if *pos >= chars.len() {
                panic!("Unexpected end of input after escape character at position {}", pos);
            }
            let escaped_char = chars[*pos];
            match escaped_char {
                'n' => result.push('\n'),
                't' => result.push('\t'),
                '\\' => result.push('\\'),
                '"' => result.push('"'),
                _ => result.push(escaped_char),
            }
        } else {
            result.push(c);
        }
        *pos += 1;
    }
    panic!("Unterminated string starting at position {}", pos);
}

fn parse_multiline_string(chars: &[char], pos: &mut usize) -> String {
    // We expect that chars[pos..pos+3] == ['"', '"', '"']
    if *pos + 2 >= chars.len() || chars[*pos] != '"' || chars[*pos + 1] != '"' || chars[*pos + 2] != '"' {
        panic!("Expected '\"\"\"' at position {}", pos);
    }
    *pos += 3; // Skip opening '"""'

    // Optionally skip a newline immediately after the opening triple quotes
    if *pos < chars.len() && chars[*pos] == '\n' {
        *pos += 1;
    }

    let mut result = String::new();
    while *pos + 2 < chars.len() {
        if chars[*pos] == '"' && chars[*pos + 1] == '"' && chars[*pos + 2] == '"' {
            *pos += 3; // Skip closing '"""'
            // Optionally remove the last newline if present
            if result.ends_with('\n') {
                result.pop();
            }
            return result;
        } else {
            result.push(chars[*pos]);
            *pos += 1;
        }
    }
    panic!("Unterminated multiline string starting at position {}", pos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_strings_content_single_line() {
        let content = r#"
"key1" = "value1"
"key2" = "value2"
"#;

        let parsed = parse_strings_content(content);
        assert_eq!(parsed.get("key1").unwrap(), "value1");
        assert_eq!(parsed.get("key2").unwrap(), "value2");
    }

    #[test]
    fn test_parse_strings_content_multiline() {
        let content = r#"
"multiline.key" = """
Line1
Line2
Line3
"""
"#;

        let parsed = parse_strings_content(content);
        assert_eq!(
            parsed.get("multiline.key").unwrap(),
            "Line1\nLine2\nLine3"
        );
    }

    #[test]
    fn test_parse_strings_content_sample() {
        let content = r#"
"example" = "value"
"multiline.example" = """
Some 
multiline 
value
"""

"some other example" = "some other value"
"#;

        let parsed = parse_strings_content(content);
        assert_eq!(parsed.get("example").unwrap(), "value");
        assert_eq!(parsed.get("some other example").unwrap(), "some other value");
        assert_eq!(
            parsed.get("multiline.example").unwrap(),
            "Some \nmultiline \nvalue"
        );
    }

    #[test]
    #[should_panic(expected = "Expected '=' after key")]
    fn test_parse_strings_content_missing_equal() {
        let content = r#"
"key1"  "value1"
"#;

        parse_strings_content(content);
    }

    #[test]
    #[should_panic(expected = "Unterminated string")]
    fn test_parse_strings_content_unterminated_string() {
        let content = r#"
"key1" = "value1
"#;

        parse_strings_content(content);
    }

    #[test]
    #[should_panic(expected = "Expected '\"' at position")]
    fn test_parse_strings_content_invalid_start() {
        let content = r#"
key1 = "value1"
"#;

        parse_strings_content(content);
    }
}
