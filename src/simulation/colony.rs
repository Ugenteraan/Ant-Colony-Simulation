
//import crates.
use glam::Vec2;
// use ant::Ant;

#[derive(Debug)]
pub struct Colony {
	pub position: Vec2,
	pub food_in_colony: u32,
}


impl Colony {


	pub fn new(x: f32, y: f32, default_food_in_colony: u32) -> Self {

		Colony {
			position: Vec2::new(x, y),
			food_in_colony: default_food_in_colony,
		}

	}


	// fn generate_id(&self) -> u32 {
	//     &self.ant_id_counter.fetch_add(1, Ordering::Relaxed)
	// }


	// pub fn spawn_initial_ants(&self, next_id:AtomicU32, food_in_store: &u32) -> VecDeque<Ant> {

	// 	let mut ants = VecDeque::new();


	// 	if (ants.len() as u32) < self.food_in_store {

	// 		let new_id: u32 = self.generate_id();

	// 		let ant = Ant::new(new_id, self.position.clone(), )
			
	// 		ants.push_back(Ant::new());
	// 	}

	// }

}