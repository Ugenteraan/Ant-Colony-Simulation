//Storage to store all the entities here.
use crate::ecs::entity_data::EntityData;

pub struct EntityComponentStorage {
    pub entity_data: Vec<EntityData>,
}

impl EntityComponentStorage {
    
    pub fn new() -> Self {
        EntityComponentStorage {
            entity_data: Vec::new() 
        }
    }


    pub fn add_entity(&mut self, entity_data: EntityData) -> () {
        self.entity_data.push(entity_data);
    }
}
