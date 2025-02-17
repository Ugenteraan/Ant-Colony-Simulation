

use crate::utils;
use bevy::prelude::Resource;
use std::collections::HashMap;
use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
	Empty,
	Ant,
	Pheromone(u8),
	Colony,
	Food,
}

#[derive(Resource, Debug)]
pub struct World {
	pub height: usize,
	pub width: usize,
	pub cell_size: f32,
	pub grid: Vec<Vec<Cell>>,
	pub colony_position: Vec2
}



impl World {
	
	pub fn new(width: usize, height: usize, cell_size: f32, colony_position: Vec2) -> Self {


		let mut world = World {
			height: height,
			width: width,
			cell_size: cell_size,
			grid: vec![vec![Cell::Empty; width]; height],
			colony_position: colony_position,
		};

		let (colony_x, colony_y) = utils::vec2_to_grid_index(colony_position, width, height, cell_size);
		world.grid[colony_x][colony_y] = Cell::Colony;

		world

	}


	pub fn get_cell(&self, position: Vec2) -> Cell {

		let (x, y) = utils::vec2_to_grid_index(position, self.width, self.height, self.cell_size);
		self.grid[x][y]

	}	

	// pub fn set_cell(&self, position:Vec2, cell: Cell) -> () {

	// }



}