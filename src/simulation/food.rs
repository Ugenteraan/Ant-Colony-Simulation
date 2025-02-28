
use rand::Rng;
use eframe::egui::Vec2;
use crate::simulation::world::Cell;
use crate::utils;

#[derive(Debug)]
pub struct Food {
	pub position: Vec2,
	pub size: f32,
}


impl Food {


	pub fn new(world_grids: &Vec<Vec<Cell>>, world_width: &usize, world_height: &usize) -> Option<Self> {

		//generate a random position and check with the grid whether it's empty or pheromone only cell.
		//if not, generate again.
		let mut random_position: Vec2 = Vec2::new(
											rand::rng().random_range(0.0..*world_width as f32) as f32,
											rand::rng().random_range(0.0..*world_height as f32) as f32
										);

		let (mut x, mut y) = utils::world_to_grid(random_position);

		let mut food_generate_counter = 0;
		loop {

			if food_generate_counter >= 10 {
				return None;
			}

			match world_grids[x][y] {
				Cell::Empty => {break;},
				_ => {

					//generate a new random position.
					random_position = Vec2::new(
											rand::rng().random_range(0.0..*world_width as f32) as f32,
											rand::rng().random_range(0.0..*world_height as f32) as f32
											);
					(x, y) = utils::world_to_grid(random_position);
					food_generate_counter += 1;
				}

			}
		}

		Some(Self {
			position: random_position,
			size: rand::rng().random_range(10.0..50.0) as f32 //random food size
		})





	}

}