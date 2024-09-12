use crate::game_engine::storage::{get_value_for_key, set_value_for_key, StorageKey};

impl StorageKey {
    fn dialogue_answer(dialogue_id: u32) -> String {
        format!("dialogue.answer.{}", dialogue_id)
    }

    fn dialogue_reward_collected(dialogue_id: u32) -> String {
        format!("dialogue.reward.{}", dialogue_id)
    }
}

pub fn set_dialogue_answer(dialogue_id: u32, answer: u32) {
    set_value_for_key(&StorageKey::dialogue_answer(dialogue_id), answer);    
}

pub fn set_dialogue_reward_collected(dialogue_id: u32) {
    set_value_for_key(&StorageKey::dialogue_reward_collected(dialogue_id), 1);    
}

pub fn has_dialogue_reward_been_collected(dialogue_id: u32) -> bool {
    if let Some(collected) = get_value_for_key(&StorageKey::dialogue_reward_collected(dialogue_id)) {
        collected == 1
    } else {
        false
    }
}