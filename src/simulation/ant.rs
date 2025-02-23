use eframe::egui::Vec2;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::utils;

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AntMode {
	Exploring,
	Returning,
}

#[derive(Debug, Clone, Copy)]
pub struct Ant {
	id: u32,
	pub position: Vec2,
	moving_direction: Vec2,
	speed: f32,
	energy: f32,
	lifespan: u32,
	carrying_food: bool,
	mode: AntMode,
	is_alive: bool,
	direction_to_colony: Vec2,

}





impl Ant {

	pub fn new(id: u32, initial_position: Vec2, initial_moving_direction: Vec2, speed: f32,
			initial_energy: f32, lifespan: u32) -> Self {

		Ant {
			id: id,
			position: initial_position,
			moving_direction: initial_moving_direction,
			speed: speed,
			energy: initial_energy,
			lifespan: lifespan,
			carrying_food: false,
			mode: AntMode::Exploring,
			is_alive: true,
			direction_to_colony: -1.0*initial_moving_direction
		}
	}



    pub fn get_ant_position(&self) -> &Vec2 {

    	return &self.position;

    }

    pub fn move_ant(&mut self, world_height: &usize, world_width: &usize) -> () {

    	let mut rng = rand::rng();

    	// generate a random direction every once in a while.
        if rng.random::<f64>() < 0.005 {
        	let direction = utils::change_direction(false); //this will gen a direction within 90 degrees.
        	self.moving_direction = direction;
        }


        self.position += self.moving_direction*self.speed;

        //check for the border of the screen.
    	let (mut x, mut y) = utils::world_to_grid(self.position);

    	while x >= *world_width - 1 || y >= *world_height - 1 || x <= 0 || y <= 0 {
    		self.moving_direction = utils::change_direction(true);
    		self.position += self.moving_direction*self.speed;

    		(x, y) = utils::world_to_grid(self.position);
    	}	

    	// let mut counter: u32 = 0;
        // while x >= *world_width - 2 || x <= 2  || y >= *world_height - 2 || y <= 2  {
    	// 	let direction = utils::change_direction(true);
    	// 	self.moving_direction = direction;
    	// 	self.position += self.moving_direction*self.speed;

    	// 	(x, y) = utils::world_to_grid(self.position); //update the x and y back.

    	// 	//if more than 5 attempts at random direction is already made.
    	// 	if counter > 10 {
    	// 		self.moving_direction = self.direction_to_colony;
    	// 		self.position += self.moving_direction*self.speed;
    	// 		(x, y) = utils::world_to_grid(self.position); //update the x and y back.
    	// 		break;
    	// 	}

    	// 	counter += 1;
    	// }

    	

    	




    }

    // pub fn set_ant_position(&mut self, new_position: Vec2) -> () {

    // 	self.position = new_position;
    // 	//calculate the direction to colony and set the direction to colony here.
    // 	return;
    // }

    // pub fn set_ant_direction(&mut self, new_direction: Vec2) -> () {

    // 	self.moving_direction = new_direction;
    // }

	
}

