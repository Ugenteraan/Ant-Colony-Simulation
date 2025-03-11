//set ups the simulation by spawning entities and components and place them into the
//EntityComponentStorage.
//Also runs the game update and other necessary loops.

use crate::ecs::entities::{Ant, Food, Nest, Pheromone, Predator};

use crate::ecs::entity_component_storage::EntityComponentStorage;
use crate::ecs::entity_data::EntityData;
use crate::ecs::entity_manager::EntityManager;
use crate::resources::world::{Cell, World};
use crate::utils;

use eframe::egui::Vec2;

pub struct Simulation<'a> {
    world: &'a mut World,
    entity_manager: EntityManager,
    entity_component_storage: EntityComponentStorage,
}

impl<'a> Simulation<'a> {
    pub fn new(world: &'a mut World) -> Self {
        Simulation {
            world: world,
            entity_manager: EntityManager::new(),
            entity_component_storage: EntityComponentStorage::new(),
        }
    }

    pub fn initialize_entities(&mut self) -> () {
        //init nest.
        let nest_position = Vec2::new(
            self.world.width as f32 / 2.0,
            self.world.height as f32 / 2.0,
        );
        let mut nest = EntityData::new(self.entity_manager.create_entity(), nest_position);
        nest.add_nest(Nest {});

        self.entity_component_storage.add_entity(nest);
    }

    pub fn update_world(&mut self) -> () {
        for entity_component in self.entity_component_storage.entity_data.iter() {
            //update the nest.
            if entity_component.nest.is_some() {
                self.world.set_grid(entity_component.position.0, Cell::Nest);
            }
        }
    }
}
