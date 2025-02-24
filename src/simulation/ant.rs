use eframe::egui::Vec2;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::utils;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AntMode {
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
	pub direction_to_colony: Vec2,

}





impl Ant {

	pub fn new(id: u32, initial_position: Vec2, initial_moving_direction: Vec2, turn_probability: f32, speed: f32, initial_energy: f32, lifespan: u32) -> Self {

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
			direction_to_colony: -1.0*initial_moving_direction
		}
	}



    pub fn get_ant_position(&self) -> &Vec2 {

    	return &self.position;

    }

    

    // pub fn set_ant_position(&mut self, new_position: Vec2) -> () {

    // 	self.position = new_position;
    // 	//calculate the direction to colony and set the direction to colony here.
    // 	return;
    // }

    // pub fn set_ant_direction(&mut self, new_direction: Vec2) -> () {

    // 	self.moving_direction = new_direction;
    // }

	
}

