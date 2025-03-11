//set ups the simulation by spawning entities and components and place them into the
//EntityComponentStorage.
//Also runs the game update and other necessary loops.

use crate::ecs::entities::{Ant, Food, Nest, Pheromone, Predator};
use crate::ecs::components::AntMode;

use crate::ecs::entity_component_storage::EntityComponentStorage;
use crate::ecs::entity_data::EntityData;
use crate::ecs::entity_manager::EntityManager;
use crate::resources::world::{Cell, World};
use crate::ecs::systems::system;
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

        let ant_position = nest_position.clone();

        let nest_capacity = 0.0;

        let mut nest = EntityData::new(self.entity_manager.create_entity(), nest_position);
        nest.add_nest(Nest {});
        nest.add_capacity(nest_capacity);

        self.entity_component_storage.add_entity(nest);

        //initialize a number of ants in the beginning.
        for _idx_ant in 0..10 {
            let velocity: Vec2 = Vec2::new(0.1, 0.1); //set ant speed here.
            let random_direction: Vec2 = utils::gen_rand_direction();
            let turning_chance: f32 = 0.001; //set turning chance here.

            let mut ant = EntityData::new(self.entity_manager.create_entity(), ant_position);
            ant.add_ant(Ant {});
            ant.add_velocity(velocity);
            ant.add_direction(random_direction);
            ant.add_turning_chance(turning_chance);
            ant.add_ant_mode(AntMode::Exploring);

            self.entity_component_storage.add_entity(ant);
        }
    }

    pub fn update_world(&mut self) -> () {
        for entity_component in self.entity_component_storage.entity_data.iter() {
            //update the nest.
            if entity_component.nest.is_some() {
                self.world.set_grid(entity_component.position.0, Cell::Nest);
            }

            //update the ant.
            if entity_component.ant.is_some() {
                self.world.set_grid(entity_component.position.0, Cell::Ant);
            }
        }
    }

    pub fn run_system(&mut self) -> () {
        system::system(&mut self.entity_component_storage);
    }


}
