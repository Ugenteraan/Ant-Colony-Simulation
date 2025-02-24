
use crate::utils;
use crate::simulation::{world::World, ant::Ant};

use eframe::egui::Vec2;
use rand::Rng;

pub fn move_ant(ant: &mut Ant, world_width: &usize, world_height: &usize) {

	let mut rng = rand::rng();

	// generate a random direction every once in a while.
    if rng.random::<f32>() < ant.turn_probability {

    	let direction = utils::change_direction(); //this will gen a direction within 90 degrees.
    	ant.moving_direction = direction;

    }

    ant.position += ant.moving_direction*ant.speed;

    let (mut x, mut y) = utils::world_to_grid(ant.position);

    //check for the border of the screen.
	//NOTE: IMPORTANT!
	//instead of assigning a totally new direction vector, we have to rotate the current direction vector!!!!!
	if x >= world_width - 1 || y >= world_height - 1 || x <= 0 || y <= 0 {

		ant.moving_direction = -1.0 * ant.moving_direction; //reverse the direction.
		ant.position += ant.moving_direction*ant.speed;

		(x, y) = utils::world_to_grid(ant.position);
	}	

}
