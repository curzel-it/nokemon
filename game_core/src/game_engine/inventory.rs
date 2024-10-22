use std::{fs::File, io::{BufReader, Write}, path::PathBuf, sync::{mpsc::{self, Sender}, RwLock}, thread};
use lazy_static::lazy_static;
use serde_json;
use crate::{constants::INVENTORY_PATH, entities::species::{species_by_id, EntityType}, game_engine::entity::Entity};

lazy_static! {
    pub static ref INVENTORY: RwLock<Vec<Entity>> = RwLock::new(load_inventory());

    static ref SAVE_THREAD: (Sender<Vec<Entity>>, thread::JoinHandle<()>) = {
        let (tx, rx) = mpsc::channel::<Vec<Entity>>();
        let file_path = INVENTORY_PATH.to_string();

        let handle = thread::spawn(move || {
            while let Ok(inventory) = rx.recv() {
                save_inventory(&file_path, &inventory);
            }
        });
        (tx, handle)
    };
}

pub fn add_to_inventory(entity: Entity) {
    if matches!(entity.entity_type, EntityType::Bundle) {
        let bundle_species = species_by_id(entity.species_id);

        for species_id in bundle_species.bundle_contents {
            let item = species_by_id(species_id).make_entity();
            add_to_inventory(item);
        }
    } else {
        {
            let mut inventory = INVENTORY.write().unwrap();
            inventory.push(entity);
        }
        let inventory = INVENTORY.read().unwrap().clone();
        let tx = &SAVE_THREAD.0;
        tx.send(inventory).expect("Failed to send inventory data to save thread");
    }
}

pub fn remove_from_inventory(id: u32) {
    {
        let mut inventory = INVENTORY.write().unwrap();
        if let Some(pos) = inventory.iter().position(|x| x.id == id) {
            inventory.remove(pos);
        }
    }

    let inventory = INVENTORY.read().unwrap().clone();
    let tx = &SAVE_THREAD.0;
    tx.send(inventory).expect("Failed to send inventory data to save thread");
}

pub fn remove_one_of_species_from_inventory(id: u32) {
    {
        let mut inventory = INVENTORY.write().unwrap();
        if let Some(pos) = inventory.iter().position(|x| x.species_id == id) {
            inventory.remove(pos);
        }
    }

    let inventory = INVENTORY.read().unwrap().clone();
    let tx = &SAVE_THREAD.0;
    tx.send(inventory).expect("Failed to send inventory data to save thread");
}

pub fn get_inventory() -> Vec<Entity> {
    let inventory = INVENTORY.read().unwrap();
    inventory.clone()
}

pub fn inventory_contains_species(species_id: u32) -> bool {
    INVENTORY.read().unwrap().iter().any(|e| e.species_id == species_id)
}

fn load_inventory() -> Vec<Entity> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push(INVENTORY_PATH);

    let file = File::open(path).expect("Failed to open inventory.json file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to deserialize inventory file from JSON")
}

fn save_inventory(path: &str, inventory: &Vec<Entity>) {
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
