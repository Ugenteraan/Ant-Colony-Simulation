//Simulation logics.

use crate::simulation::world::{World, Cell};
use crate::simulation::{ant::{Ant, AntMode}, food::Food, pheromone::Pheromone};
use crate::utils::{gen_rand_direction, world_to_grid};
use crate::simulation::ant_movement::move_ant;
use eframe::egui::Vec2;
use rand::Rng;



pub struct System {
	ant_energy: f32,
	ant_lifespan: u32,
	ant_speed: f32,
	ant_turn_probability: f32,
	food_spawn_rate: f32,
	ant_weak_pheromone_intensity: f32,
	ant_strong_pheromone_intensity: f32
}


impl System {

	pub fn new(ant_energy: f32, ant_lifespan: u32, ant_speed: f32, ant_turn_probability: f32, food_spawn_rate: f32, ant_weak_pheromone_intensity: f32, ant_strong_pheromone_intensity: f32) -> Self {

			System {
				ant_energy: ant_energy,
				ant_lifespan: ant_lifespan,
				ant_speed: ant_speed,
				ant_turn_probability: ant_turn_probability,
				food_spawn_rate: food_spawn_rate,
				ant_weak_pheromone_intensity: ant_weak_pheromone_intensity,
				ant_strong_pheromone_intensity: ant_strong_pheromone_intensity
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

 				world.colony.insert_ant(position, direction, self.ant_turn_probability, self.ant_speed, self.ant_energy, self.ant_lifespan, self.ant_weak_pheromone_intensity, self.ant_strong_pheromone_intensity);

 			}
 		}

 		//randomly spawn foods.
		if rand::rng().random::<f32>() < self.food_spawn_rate {

			let food = Food::new(&world.grid, &world.width, &world.height);

	    	match food {
	    		Some(food) => {

	    			world.spawn_food(food);

	    		},
	    		None => {}
	    	}

	    }

 		for i in 0..world.colony.ants.len() {

 			//mark the current ant-filled cell empty before marking the other as filled with ant.
 			//we want to avoid marking other cell such as food or pheromone 
 			let (x, y) = world_to_grid(world.colony.ants[i].position);

 			match world.grid[x][y] {
 				Cell::Ant => {
 					world.set_cell(world.colony.ants[i].position, Cell::Empty); 
 				},
 				Cell::Food => {
 					world.colony.ants[i].mode = AntMode::Returning;
 				}
 				// Cell::Ant_Pheromone => {
 				// 	world.set_cell(world.colony.ants[i].position, Cell::Pheromone); 
 				// }
 				_ => {}

 			}
 			

 			if let Some(ant) = world.colony.ants.get_mut(i) {
 				move_ant(ant, &world.width, &world.height, &world.grid, &world.colony.position);

 				let ant_position = ant.position;
 				world.set_cell(ant_position, Cell::Ant); //mark the cell after the move with ant.
 			}

 			
 			world.set_pheromone(&i); //set pheromone of the ant.


 		}
 	

	}
}