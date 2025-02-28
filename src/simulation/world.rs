
use crate::simulation::{ant::{Ant, AntMode}, colony::Colony, food::Food, pheromone::{Pheromone}};
use crate::utils;
use eframe::egui::Vec2;
use std::collections::{HashMap, VecDeque};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
	Empty,
	Ant,
	Colony,
	Food
}

#[derive(Debug)]
pub struct World {
	pub height: usize,
	pub width: usize,
	pub grid: Vec<Vec<Cell>>,
	pub colony: Colony, //code the whole colony to be spawned here tomorrow.
	pub foods: VecDeque<Food>,
	pub pheromones: HashMap<(usize, usize), Pheromone>
}



impl World {
	
	pub fn new(width: usize, height: usize, colony_position: Vec2, default_food_in_colony: u32) -> Self {

		
		let colony = Colony::new(colony_position.x, colony_position.y, default_food_in_colony);
		let (colony_x, colony_y) = utils::world_to_grid(colony.position);

		let mut world = World {
			height: height,
			width: width,
			grid: vec![vec![Cell::Empty; width]; height],
			colony: colony,
			foods: VecDeque::new(),
			pheromones: HashMap::new()
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

	pub fn spawn_food(&mut self, food: Food) -> () {

		self.set_cell(food.position, Cell::Food);
		self.foods.push_back(food);
	}

	pub fn set_pheromone(&mut self, ant_idx: &usize) -> () {

		

		let ant: &Ant = &self.colony.ants[*ant_idx];

		let (x, y) = utils::world_to_grid(ant.position); //convert the pos to grid first.

		if let Some(pheromone) = self.pheromones.get_mut(&(x,y)) {

			match ant.mode {
				AntMode::Exploring => {
					pheromone.intensity += ant.weak_pheromone_intensity;
				},
				AntMode::Returning => {

					if ant.carrying_food { //ants can also return when the energy is low.

						pheromone.intensity += ant.strong_pheromone_intensity;
					}
				}
			}

		} else {

			match ant.mode {
				AntMode::Exploring => {
					self.pheromones.insert((x, y), Pheromone::new((x, y), ant.weak_pheromone_intensity));
				},
				AntMode::Returning => {

					if ant.carrying_food {
						self.pheromones.insert((x, y), Pheromone::new((x, y), ant.strong_pheromone_intensity));
					}
				}
			}

		}

	}


}










