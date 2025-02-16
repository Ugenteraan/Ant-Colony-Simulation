
#[derive(Debug, Clone)]
pub enum Cell {
	Empty,
	Ant,
	Pheromone(u8),
	Colony,
	Food,
}

#[derive(Debug)]
pub struct World {
	height: usize,
	width: usize,
	grid: Vec<Vec<Cell>>,
}



impl World {
	
	pub fn new(width: usize, height: usize) -> Self {

		World {
			height: height,
			width: width,
			grid: vec![vec![Cell::Empty; width]; height],
		}

	}


	pub fn get_cell(&self, x: usize, y: usize) -> &Cell {

		if x < self.width && y < self.height {

			return &self.grid[x][y];
		}

		&Cell::Empty

	}	




}