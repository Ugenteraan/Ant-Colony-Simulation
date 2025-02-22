//Simulation logics.

use crate::simulation::{world::World};
use crate::utils::{gen_rand_direction};
use eframe::egui::Vec2;




pub struct System {
	pub ant_energy: f32,
	pub ant_lifespan: u32,
}


impl System {

	pub fn new(ant_energy: f32, ant_lifespan: u32) -> Self {

		System {
			ant_energy: ant_energy,
			ant_lifespan: ant_lifespan
		}

	}

	pub fn update_world(&mut self, world: &mut World) -> () {
 		
 		let food_in_colony = &world.colony.food_in_colony;
 		//there's no ants but colony has food (possibly the start of the game.)
 		if *food_in_colony > 0 && world.colony.ants.len() == 0 {

 			//spawn ants based on the food in the colony.
 			for _i in 0..*food_in_colony {

 				//a random direction for each ant.
 				let direction: Vec2 = gen_rand_direction();
 				let position: Vec2 = world.colony.position + direction;

 				world.colony.insert_ant(position, direction, self.ant_energy, self.ant_lifespan);

 			}

 		}
 		for ant in world.colony.ants.iter_mut() {
 			ant.move_ant();
 		}

	}
}