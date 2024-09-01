use crate::lang::localizable::LocalizableText;

pub fn localized_dialogue(dialogue_id: u32) -> String {
    format!("dialogue.{}", dialogue_id).localized()
}