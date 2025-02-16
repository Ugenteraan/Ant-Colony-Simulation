use glam::Vec2;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AntMode {
	Exploring,
	Returning,
}

pub struct Ant {
	id: u32,
	position: Vec2,
	moving_direction: Vec2,
	energy: f32,
	lifespan: u32,
	carrying_food: bool,
	mode: AntMode,
	is_alive: bool,
	direction_to_colony: Vec2,
}



pub impl Ant {

	pub fn new(id: u32, initial_position: Vec2, initial_moving_direction: Vec2, 
			initial_energy: f32, lifespan: u32) -> Self {

		Ant {
			id: id,
			position: initial_position,
			moving_direction: initial_moving_direction,
			energy: initial_energy,
			lifespan: lifespan,
			carrying_food: false,
			mode: AntMode::Exploring,
			is_alive: true,
			direction_to_colony: -1*initial_moving_direction
		}
	}

    pub fn get_ant_position(&self) -> &Vec2 {

    	return self.position;

    }

    pub fn set_ant_position(&mut self, new_position: Vec2) -> () {

    	self.position = new_position;
    	//calculate the direction to colony and set the direction to colony here.
    	return;
    }

    pub fn set_ant_direction(&mut self, new_direction: Vec2) -> () {

    	self.moving_direction = new_direction;
    }

	
}

