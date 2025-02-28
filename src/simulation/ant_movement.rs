
use crate::utils;
use crate::simulation::{world::World, world::Cell, ant::{Ant, AntMode}};

use eframe::egui::Vec2;
use rand::Rng;



fn apply_new_direction(ant: &mut Ant, border: bool, blocked: bool) -> Vec2 {

	let (cos_angle, sin_angle) = utils::change_direction(border, blocked); //this will gen a direction within 90 degrees
    	
	let current_x = ant.moving_direction.x;
	let current_y = ant.moving_direction.y;

	let new_x = current_x * cos_angle + (current_y * -1.0*sin_angle);
	let new_y = current_x * sin_angle + current_y * cos_angle;

	return Vec2::new(new_x, new_y);

}


fn wandering_ant(ant: &mut Ant, world_width: &usize, world_height: &usize, world_grids: &Vec<Vec<Cell>>, colony_position: &Vec2) {

	let mut rng = rand::rng();

	// generate a random direction every once in a while.
    if rng.random::<f32>() < ant.turn_probability {

    	ant.moving_direction = apply_new_direction(ant, false, false);
    }

    let mut new_position = ant.position + ant.moving_direction*ant.speed; //update the ant's new position either with the random direction above or the current direction.

    
    let (mut x, mut y) = utils::world_to_grid(new_position); //get the x and y of the current grid now after the above update.

    //if the ants are very nearby to the colony, then skip all the logics below.
    //we don't want collision detection nearby colony, else the spawning part will be stuck.
    let ant_to_colony_vector = *colony_position - ant.position;
    let distance_to_colony = ant_to_colony_vector.length(); //length of the vector a.k.a distance.
    
    if distance_to_colony > -1.0 && distance_to_colony < 1.0 {
    	ant.position += ant.moving_direction*ant.speed; //update the position.
    	return;
    }

    //check for the borders of the screen.
	if x >= world_width - 1 || y >= world_height - 1 || x <= 0 || y <= 0 {

    	ant.moving_direction = apply_new_direction(ant, true, false);

    	
    	new_position = ant.moving_direction*ant.speed; //update the new position if this if block is executed.
    	(x, y) = utils::world_to_grid(ant.position); //update x and y

	}

	let mut blocked_counter = 1;

	//we check if the current grip that the ant moved is valid or not in this loop
	loop {

		let current_grid = world_grids[x][y];
		match current_grid {
			Cell::Ant => {
				ant.moving_direction = apply_new_direction(ant, true, true);
				new_position = ant.position + ant.moving_direction*ant.speed;
				(x, y) = utils::world_to_grid(new_position);
				blocked_counter += 1;
			},
			_ => {break;}
		}

		if blocked_counter >= 7 { //we pick 7 since we turn 45 degrees everytime and 7 x 45 = 315 degrees. Right before one more turn to come back where we started.

			return; //no need to update the ant's position if we can't get a solution to this blocked grid.
		}
	}
	ant.position += ant.moving_direction*ant.speed; //finally update the position.
}


// fn returning_ant(ant: &mut Ant, world_width: &usize, world_height: &usize, world_grids: &Vec<Vec<Cell>>, colony_position: &Vec2) {


// }



pub fn move_ant(ant: &mut Ant, world_width: &usize, world_height: &usize, world_grids: &Vec<Vec<Cell>>, colony_position: &Vec2) -> () {


	let ant_mode = ant.mode;

	match ant_mode {

		AntMode::Exploring => { wandering_ant(ant, world_width, world_height, world_grids, colony_position); },
		AntMode::Returning => { }

	}

	

}



















