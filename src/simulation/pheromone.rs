



#[derive(Debug)]
pub struct Pheromone {
	pub id: u32, //different colonies have different scent.
	pub position: (usize, usize), //pheromones are stored in the grid-based coordinate.
	pub intensity: f32
}



impl Pheromone {

	pub fn new(position: (usize, usize), intensity: f32) -> Self {

		Pheromone {
			id: 1, //since we only have 1 colony.
			position: position,
			intensity: intensity
		}

	}

}