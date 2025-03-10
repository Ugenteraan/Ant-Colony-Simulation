// Manager to create entities with unique IDs.

use std::sync::atomic::{AtomicU32, Ordering};

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub struct EntityID(u32);

pub struct EntityManager {
    next_id : AtomicU32,
}


impl EntityManager {
    
    pub fn new() -> Self {
        Self {next_id: AtomicU32::new(0)}
    }

    pub fn create_entity(&mut self) -> EntityID {
    
        let new_id: u32 = self.next_id.fetch_add(1, Ordering::Relaxed);
        EntityID(new_id)
    }
}


