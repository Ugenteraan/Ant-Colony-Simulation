
use eframe::egui::Vec2;
use crate::simulation::world::Cell;

#[Derive(Debug)]
struct Food {
	position: Vec2,
	size: f32,
}


impl Food {


	pub fn new(world_grids: &Vec<Vec<Cell>>) -> Self {

		//generate a random position and check with the grid whether it's empty or pheromone only cell.
		//if not, generate again.

	}

}