
use crate::simulation::colony;
use colony::Colony;
use crate::utils;
use eframe::egui::Vec2;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
	Empty,
	Ant,
	Pheromone(u8),
	Colony,
	Food,
	Ant_Pheromone(u8), //cells can contain ant and pheromone.
}

#[derive(Debug)]
pub struct World {
	pub height: usize,
	pub width: usize,
	pub grid: Vec<Vec<Cell>>,
	pub colony: Colony, //code the whole colony to be spawned here tomorrow.

}



impl World {
	
	pub fn new(width: usize, height: usize, colony_position: Vec2, default_food_in_colony: u32) -> Self {

		
		let colony = Colony::new(colony_position.x, colony_position.y, default_food_in_colony);
		let (colony_x, colony_y) = utils::world_to_grid(colony.position);

		let mut world = World {
			height: height,
			width: width,
			grid: vec![vec![Cell::Empty; width]; height],
			colony: colony
		};

		//mark the grid as the colony grid after converting the Vec2 to x and y position.
		world.grid[colony_x][colony_y] = Cell::Colony;
		return world;

	}


	pub fn get_cell(&self, position: Vec2) -> Cell {

		let (x, y) = utils::world_to_grid(position);
		self.grid[x][y]

	}	


	pub fn set_cell(&mut self, position: Vec2, cell: Cell) -> () {

		let (x, y) = utils::world_to_grid(position);
		self.grid[x][y] = cell;
	}





}