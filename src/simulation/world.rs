
use crate::simulation::{ant, colony};
use ant::Ant;
use colony::Colony;
use crate::utils;
use bevy::prelude::Resource;
use std::collections::VecDeque;
use glam::Vec2;
use std::sync::atomic::{AtomicU32, Ordering};


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
	pub colony: Colony, //code the whole colony to be spawned here tomorrow.
	pub ant_id_counter: AtomicU32,
	pub ants: VecDeque<Ant>
}



impl World {
	
	pub fn new(width: usize, height: usize, cell_size: f32, colony_position: Vec2, default_food_in_colony: u32) -> Self {

		let mut initial_ant_id_counter: AtomicU32 = AtomicU32::new(1);
		let colony = Colony::new(colony_position.x, colony_position.y, default_food_in_colony);

		let mut world = World {
			height: height,
			width: width,
			cell_size: cell_size,
			grid: vec![vec![Cell::Empty; width]; height],
			colony: colony,
			ant_id_counter: initial_ant_id_counter,
			ants: VecDeque::new(),
		};

		//mark the grid as the colony grid after converting the Vec2 to x and y position.
		let (colony_x, colony_y) = utils::vec2_to_grid_index(colony_position, width, height, cell_size);
		world.grid[colony_x][colony_y] = Cell::Colony;

		return world;

	}

	pub fn insert_ant(&mut self, position: Vec2, moving_direction: Vec2, energy: f32, lifespan: u32){

		//generate a new id for the ant.
		let new_id: u32 = self.ant_id_counter.fetch_add(1, Ordering::Relaxed);

		let ant: Ant = Ant::new(new_id, position, moving_direction, energy, lifespan);

		self.ants.push_back(ant);

	}


	pub fn get_cell(&self, position: Vec2) -> Cell {

		let (x, y) = utils::vec2_to_grid_index(position, self.width, self.height, self.cell_size);
		self.grid[x][y]

	}	

	// pub fn set_cell(&self, position:Vec2, cell: Cell) -> () {
		
	// }



}