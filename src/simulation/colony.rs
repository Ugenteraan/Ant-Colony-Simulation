
//import crates.
use glam::Vec2;
use std::sync::atomic::{AtomicU32, Ordering};
// use ant::Ant;


pub struct Colony {
	ant_id_counter: AtomicU32,
	position: Vec2,
	food_in_store: u32,
}


impl Colony {


	fn new(x: f32, y: f32, default_food_in_store: u32, initial_ant_id_counter: AtomicU32) -> Self {

		Colony {
			ant_id_counter: initial_ant_id_counter,
			position: Vec2::new(x, y),
			food_in_store: default_food_in_store,
		}

	}


	// fn generate_id(&self) -> u32 {
	//     &self.ant_id_counter.fetch_add(1, Ordering::Relaxed)
	// }


	// pub fn spawn_initial_ants(&self, next_id:AtomicU32, food_in_store: &u32) -> VecDeque<Ant> {

	// 	let mut ants = VecDeque::new();


	// 	if (ants.len() as u32) < self.food_in_store {

	// 		let new_id: u32 = self.generate_id();

	// 		let ant = Ant::new(new_id, self.position.clone(), )
			
	// 		ants.push_back(Ant::new());
	// 	}

	// }

}