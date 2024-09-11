use std::{fs::File, io::{BufReader, Write}, sync::{mpsc::{self, Sender}, RwLock}, thread};
use lazy_static::lazy_static;
use serde_json;
use crate::constants::INVENTORY_PATH;

lazy_static! {
    pub static ref INVENTORY: RwLock<Vec<u32>> = RwLock::new(load_inventory(INVENTORY_PATH));

    static ref SAVE_THREAD: (Sender<Vec<u32>>, thread::JoinHandle<()>) = {
        let (tx, rx) = mpsc::channel::<Vec<u32>>();
        let file_path = INVENTORY_PATH.to_string();

        let handle = thread::spawn(move || {
            while let Ok(inventory) = rx.recv() {
                save_inventory(&file_path, &inventory);
            }
        });
        (tx, handle)
    };
}

pub fn add_to_inventory(species_id: u32) {
    {
        let mut inventory = INVENTORY.write().unwrap();
        if !inventory.contains(&species_id) {
            inventory.push(species_id);
        }
    }

    let inventory = INVENTORY.read().unwrap().clone();
    let tx = &SAVE_THREAD.0;
    tx.send(inventory).expect("Failed to send inventory data to save thread");
}

pub fn remove_from_inventory(species_id: u32) {
    {
        let mut inventory = INVENTORY.write().unwrap();
        if let Some(pos) = inventory.iter().position(|&x| x == species_id) {
            inventory.remove(pos);
        }
    }

    let inventory = INVENTORY.read().unwrap().clone();
    let tx = &SAVE_THREAD.0;
    tx.send(inventory).expect("Failed to send inventory data to save thread");
}

pub fn get_inventory() -> Vec<u32> {
    let inventory = INVENTORY.read().unwrap();
    inventory.clone()
}

pub fn inventory_contains(item: u32) -> bool {
    INVENTORY.read().unwrap().contains(&item)
}

fn load_inventory(file_path: &str) -> Vec<u32> {
    let file = File::open(file_path).expect("Failed to open inventory.json file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to deserialize inventory file from JSON")
}

fn save_inventory(path: &str, inventory: &Vec<u32>) {
    if let Ok(serialized_inventory) = serde_json::to_string_pretty(inventory) {
        if let Ok(mut file) = File::create(path) {
            if let Err(e) = file.write_all(serialized_inventory.as_bytes()) {
                eprintln!("Failed to write inventory file: {}", e);
            } else {
                println!("Inventory saved successfully to {}", path);
            }
        } else {
            eprintln!("Failed to create inventory file");
        }
    } else {
        eprintln!("Failed to serialize inventory data");
    }
}
