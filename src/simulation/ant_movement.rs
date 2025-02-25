
use crate::utils;
use crate::simulation::{world::World, ant::Ant};

use eframe::egui::Vec2;
use rand::Rng;

pub fn move_ant(ant: &mut Ant, world_width: &usize, world_height: &usize) {

	let mut rng = rand::rng();

	// generate a random direction every once in a while.
    if rng.random::<f32>() < ant.turn_probability {

    	let (cos_angle, sin_angle) = utils::change_direction(false); //this will gen a direction within 90 degrees
    	
    	let current_x = ant.moving_direction.x;
    	let current_y = ant.moving_direction.y;

    	let new_x = current_x * cos_angle + (current_y * -1.0*sin_angle);
    	let new_y = current_x * sin_angle + current_y * cos_angle;

    	ant.moving_direction = Vec2::new(new_x, new_y);

    }

    ant.position += ant.moving_direction*ant.speed;

    let (mut x, mut y) = utils::world_to_grid(ant.position);

    //check for the border of the screen.
	//NOTE: IMPORTANT!
	//instead of assigning a totally new direction vector, we have to rotate the current direction vector!!!!!
	if x >= world_width - 1 || y >= world_height - 1 || x <= 0 || y <= 0 {

		let (cos_angle, sin_angle) = utils::change_direction(true);

		let current_x = ant.moving_direction.x;
    	let current_y = ant.moving_direction.y;

    	let new_x = current_x * cos_angle + (current_y * -1.0*sin_angle);
    	let new_y = current_x * sin_angle + current_y * cos_angle;

    	ant.moving_direction = Vec2::new(new_x, new_y);
		ant.position += ant.moving_direction*ant.speed;

		(x, y) = utils::world_to_grid(ant.position);
	}	

}
