use crate::constants::DEFAULT_LANG;

use super::localized_strings::LOCALIZED_STRINGS;

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