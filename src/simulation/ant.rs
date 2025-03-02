use eframe::egui::Vec2;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::simulation::pheromone::Pheromone;
use crate::simulation::world::Vec2Key;
use crate::utils;
use std::collections::HashMap;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntMode {
	Exploring,
	Returning,
}

#[derive(Debug, Clone, Copy)]
pub struct Ant {
	pub id: u32,
	pub position: Vec2,
	pub moving_direction: Vec2,
	pub turn_probability: f32,
	pub speed: f32,
	pub energy: f32,
	pub lifespan: u32,
	pub carrying_food: bool,
	pub mode: AntMode,
	pub is_alive: bool,
	pub strong_pheromone_intensity: f32, //max pheromone intensity that an ant can produce.
	pub weak_pheromone_intensity: f32,
	pub visited_pheromones: [Option<Vec2>; 5], //array of size 2.
}





impl Ant {

	pub fn new(id: u32, initial_position: Vec2, initial_moving_direction: Vec2, turn_probability: f32, speed: f32, initial_energy: f32, lifespan: u32, strong_pheromone_intensity: f32, weak_pheromone_intensity: f32) -> Self {

		Ant {
			id: id,
			position: initial_position,
			moving_direction: initial_moving_direction,
			turn_probability: turn_probability,
			speed: speed,
			energy: initial_energy,
			lifespan: lifespan,
			carrying_food: false,
			mode: AntMode::Exploring,
			is_alive: true,
			strong_pheromone_intensity: strong_pheromone_intensity,
			weak_pheromone_intensity: weak_pheromone_intensity,
			visited_pheromones: [None, None, None, None, None]
		}
	}



    pub fn get_ant_position(&self) -> &Vec2 {

    	return &self.position;

    }

    pub fn update_visited_pheromone(&mut self, pheromones: &HashMap<Vec2Key, Pheromone>) -> () {

    	let current_pheromone = pheromones.get(&Vec2Key(self.position));


    	if let Some(current_pheromone) = current_pheromone {
    	
	    	if let Some(visited_pheromone) = self.visited_pheromones[0] {

	    		self.visited_pheromones[1] = Some(visited_pheromone);
	    		self.visited_pheromones[0] = Some(current_pheromone.position);
	    	} else{
	    		self.visited_pheromones[0] = Some(current_pheromone.position);
	    	}
	    }
	    else {
	    	return;
	    }
    }

	
}

