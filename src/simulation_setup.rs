//set ups the simulation by spawning entities and components and place them into the
//EntityComponentStorage.
//Also runs the game update and other necessary loops.

use crate::ecs::entities::{Ant, Pheromone, Food, Nest, Predator};

use crate::ecs::entity_manager::EntityManager;
use crate::ecs::entity_component_storage::EntityComponentStorage;
use crate::ecs::entity_data::EntityData;
use crate::resources::world::{World, Cell};

use eframe::egui::Vec2;

pub struct Simulation {
    world: World,
    entity_manager: EntityManager,
    entity_component_storage: EntityComponentStorage,

}   



impl Simulation {
    
    pub fn new(world_width: usize, world_height: usize) -> Self {
        
        Simulation {
            world: World::new(world_width, world_height),
            entity_manager: EntityManager::new(),
            entity_component_storage: EntityComponentStorage::new(),
        }
    }

    pub fn get_world(&mut self) -> World {
        self.world.clone()
    }

    pub fn initialize_entities(&mut self) -> () {
        
        //init nest.
        let nest_position = Vec2::new(self.world.width as f32/2.0, self.world.height as f32/2.0);
        let mut nest = EntityData::new(self.entity_manager.create_entity(), nest_position);
        nest.add_nest(Nest {});
    }


        

}













