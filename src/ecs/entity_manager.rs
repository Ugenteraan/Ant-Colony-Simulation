// Manager to create entities with unique IDs.

use std::sync::atomic::{AtomicU32, Ordering};

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub struct Entity(u32);

pub struct EntityManager {
    next_id : AtomicU32,
}


impl EntityManager {
    
    pub fn new() -> Self {
        Self {next_id: AtomicU32::new(0)}
    }

    pub fn create_entity(&mut self) -> Entity {
    
        let new_id: u32 = self.next_id.fetch_add(1, Ordering::Relaxed);
        Entity(new_id)
    }
}


