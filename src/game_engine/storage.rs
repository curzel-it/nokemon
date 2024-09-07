use std::{collections::HashMap, fs::File, io::{BufReader, Write}, sync::{mpsc::{self, Sender}, RwLock}, thread};
use lazy_static::lazy_static;
use crate::constants::KEY_VALUE_STORAGE_PATH;

pub struct StorageKey {}

impl StorageKey {
    pub fn dialogue_answer(dialogue_id: u32) -> String {
        format!("dialogue.answer.{}", dialogue_id)
    }
}

fn load_stored_values(file_path: &str) -> HashMap<String, u32> {
    let file = File::open(file_path).expect("Failed to open save.json file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to deserialize save file from JSON")
}

fn save_stored_values(path: &str, data: &HashMap<String, u32>) {
    if let Ok(serialized_world) = serde_json::to_string_pretty(data) {
        if let Ok(mut file) = File::create(path) {
            if let Err(e) = file.write_all(serialized_world.as_bytes()) {
                eprintln!("Failed to write save file: {}", e);
            } else {
                println!("Data saved successfully to {}", path);
            }
        } else {
            eprintln!("Failed to create save file");
        }
    } else {
        eprintln!("Failed to serialize data");
    }
}

lazy_static! {
    static ref KEY_VALUE_STORAGE: RwLock<HashMap<String, u32>> = RwLock::new(load_stored_values(KEY_VALUE_STORAGE_PATH));
    
    static ref SAVE_THREAD: (Sender<HashMap<String, u32>>, thread::JoinHandle<()>) = {
        let (tx, rx) = mpsc::channel::<HashMap<String, u32>>();
        let file_path = KEY_VALUE_STORAGE_PATH.to_string();
        
        let handle = thread::spawn(move || {
            while let Ok(data) = rx.recv() {
                save_stored_values(&file_path, &data);
            }
        });
        
        (tx, handle)
    };
}

pub fn get_value_for_key(key: &String) -> Option<u32> {
    let storage = KEY_VALUE_STORAGE.read().unwrap();
    storage.get(key).cloned()
}

pub fn set_value_for_key(key: String, value: u32) {
    {
        let mut storage = KEY_VALUE_STORAGE.write().unwrap();
        storage.insert(key, value);
    }
    let storage = KEY_VALUE_STORAGE.read().unwrap().clone();
    let tx = &SAVE_THREAD.0;
    tx.send(storage).expect("Failed to send data to save thread");
}
