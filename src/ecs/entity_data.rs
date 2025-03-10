
use crate::ecs::entity_manager::{EntityManager, EntityID};
use crate::ecs::entities::{Ant, Pheromone, Food, Nest, Predator};
use crate::ecs::components::{Position, Velocity, Intensity, Direction, TurningChance, Capacity, AntMode};

use eframe::egui::Vec2;

struct EntityData{
    pub entity_id: EntityID,
    pub ant: Option<Ant>,
    pub pheromone: Option<Pheromone>,
    pub food: Option<Food>,
    pub nest: Option<Nest>,
    pub predator: Option<Predator>,
    pub position: Position,
    pub velocity: Option<Velocity>,
    pub intensity: Option<Intensity>,
    pub direction: Option<Direction>,
    pub turning_chance: Option<TurningChance>,
    pub capacity: Option<Capacity>,
    pub ant_mode: Option<AntMode>,
}


impl EntityData {
    
    pub fn new(entity_id: EntityID, position: Vec2) -> Self {
        
        EntityData {
            entity_id: entity_id,
            ant: None,
            pheromone: None,
            food: None,
            nest: None,
            predator: None,
            position: Position(position),
            velocity: None,
            intensity: None,
            direction: None,
            turning_chance: None,
            capacity: None,
            ant_mode: None,

        }
    }

    pub fn add_velocity(&mut self, velocity: Vec2) -> () {
        self.velocity = Some(Velocity(velocity));
    }

    pub fn add_intensity(&mut self, intensity: f32) -> () {
        self.intensity = Some(Intensity(intensity));
    }

    pub fn add_direction(&mut self, direction: Vec2) -> () {
        self.direction = Some(Direction(direction));
    }

    pub fn add_turning_chance(&mut self, turning_chance: f32) -> () {
        self.turning_chance = Some(TurningChance(turning_chance));
    }

    pub fn add_capacity(&mut self, capacity: f32) -> () {
        self.capacity = Some(Capacity(capacity));
    }

    pub fn add_ant_mode(&mut self, ant_mode: AntMode) -> () {
        self.ant_mode = Some(ant_mode);
    }

}


