use std::sync::atomic::{AtomicU32, Ordering};

static mut NEXT_BUILDING_INDEX: AtomicU32 = AtomicU32::new(1_000);
static mut NEXT_ENTITY_INDEX: AtomicU32 = AtomicU32::new(1_000_000);

pub fn get_next_entity_id() -> u32 {
    unsafe { 
        NEXT_BUILDING_INDEX.fetch_add(1, Ordering::SeqCst);
        NEXT_BUILDING_INDEX.load(Ordering::SeqCst)
    }
}

pub fn get_next_building_id() -> u32 {
    unsafe {
        NEXT_ENTITY_INDEX.fetch_add(1, Ordering::SeqCst);
        NEXT_ENTITY_INDEX.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use crate::game_engine::entity_factory::{get_next_building_id, get_next_entity_id};
    
    #[test]
    fn next_entity_id_automatically_increments() {
        let id1 = get_next_entity_id();
        let id2 = get_next_entity_id();
        let id3 = get_next_entity_id();
        assert!(id1 < id2 && id2 < id3);
    }
    
    #[test]
    fn next_building_id_automatically_increments() {
        let id1 = get_next_building_id();
        let id2 = get_next_building_id();
        let id3 = get_next_building_id();
        assert!(id1 < id2 && id2 < id3);
    }
}