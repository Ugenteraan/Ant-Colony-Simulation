

use crate::simulation::ant::Ant;

//import crates.
use eframe::egui::Vec2;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};



#[derive(Debug)]
pub struct Colony {
	pub position: Vec2,
	pub food_in_colony: u32,
	pub ant_id_counter: AtomicU32,
	pub ants: VecDeque<Ant>,
}


impl Colony {


	pub fn new(x: f32, y: f32, default_food_in_colony: u32) -> Self {

		let mut initial_ant_id_counter: AtomicU32 = AtomicU32::new(1);

		let empty_ants = VecDeque::new();

		Colony {
			position: Vec2::new(x, y),
			food_in_colony: default_food_in_colony,
			ant_id_counter: initial_ant_id_counter,
			ants: empty_ants,

		}

	}

	pub fn insert_ant(&mut self, position: Vec2, moving_direction: Vec2, energy: f32, lifespan: u32){

		//generate a new id for the ant.
		let new_id: u32 = self.ant_id_counter.fetch_add(1, Ordering::Relaxed);

		let ant: Ant = Ant::new(new_id, position, moving_direction, energy, lifespan);

		self.ants.push_back(ant);

	}

}