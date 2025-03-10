//Handles the relationship between Entity, Components and System.

use ecs::{component_storage::ComponentStorage, entity_manager::EntityManager};
use ecs::components::{Position};
use resources::{world::World};


pub struct Simulation {
    world: World,
    entity_manager: EntityManager,
    component_storage: ComponentStorage
}   



impl Simulation {
    
    pub fn new(&self, world_width: usize, world_height: usize) -> Self {
       
        let world: World = World::new(world_width, world_height);
        let mut entity_manager: EntityManager = EntityManager::new();
        let mut component_storage: ComponentStorage = ComponentStorage::new();

        Simulation{
            world: world,
            entity_manager: entity_manager,
            component_storage: component_storage,
        }
    }

    pub add_entities(&mut self) -> () {
    
        

    }


}













