
use crate::utils;
use crate::simulation::world::{World, Cell, Vec2Key};
use crate::simulation::ant::{Ant, AntMode};
use crate::simulation::pheromone::Pheromone;
use std::collections::HashMap;


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


fn find_strongest_pheromone<'a>(ant: &'a Ant, pheromones: &'a HashMap<Vec2Key, Pheromone>, ant_pheromone_radius: &'a f32) -> Option<&'a Pheromone> {

	//collect all the nearby pheromones.
	let filtered_pheromones: Vec<&Pheromone> = pheromones
													.iter()
													.filter(|(_, pheromone)| utils::calculate_distance(ant.position, pheromone.position) <             *ant_pheromone_radius)
													.map(|(_, pheromone )| pheromone)
													.collect(); 
	


	let mut strongest_pheromone: Option<&Pheromone> = None; //to keep track of the strongest one.
	

	for nearby_pheromone in filtered_pheromones {

		//check if the pheromone already visited before.
		let exists: bool = ant.visited_pheromones.iter().any(|&x| x == Some(nearby_pheromone.position));

		if exists {
			continue;
		} else {

			//update the strongest pheromone.
			if let Some(current_strongest) = strongest_pheromone {

				if current_strongest.intensity < nearby_pheromone.intensity {
					strongest_pheromone = Some(nearby_pheromone);
				}
				continue;

			} else{

				strongest_pheromone = Some(nearby_pheromone);
			}

		}
	}

	strongest_pheromone

}



fn wandering_ant(ant: &mut Ant, world_width: &usize, world_height: &usize, world_grids: &Vec<Vec<Cell>>, colony_position: &Vec2, returning_ant: bool, pheromones: &HashMap<Vec2Key, Pheromone>, ant_pheromone_radius: &f32) {



	let strongest_pheromone: Option<&Pheromone> = find_strongest_pheromone(&ant, pheromones, ant_pheromone_radius);



	if let Some(strongest_pheromone) = strongest_pheromone {
		let direction_to_pheromone: Vec2 =  strongest_pheromone.position - ant.position;

		
		if direction_to_pheromone.length() != 0.0 {
			if returning_ant{
				println!("Previous direction: {:?}", ant.moving_direction);
				let normalized_direction: Vec2 = direction_to_pheromone/direction_to_pheromone.length(); //normalizing to unit vector.
				ant.moving_direction = normalized_direction;
				println!("Direction changed! {:?}", normalized_direction);
			} else{

				if strongest_pheromone.intensity > 5.0 {
					let normalized_direction: Vec2 = direction_to_pheromone/direction_to_pheromone.length(); //normalizing to unit vector.
					ant.moving_direction = normalized_direction;
				}
			}

		} 


	}


	let mut rng = rand::rng();

	// generate a random direction every once in a while.
	// the probability is lower if the ant is a returning ant.
    if !returning_ant && rng.random::<f32>() < ant.turn_probability {

    	ant.moving_direction = apply_new_direction(ant, false, false);

    } else if returning_ant && rng.random::<f32>() < ant.turn_probability * 0.1{ 
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




fn returning_ant(ant: &mut Ant, world_width: &usize, world_height: &usize, world_grids: &Vec<Vec<Cell>>, colony_position: &Vec2, pheromones: &HashMap<Vec2Key, Pheromone>, ant_pheromone_radius: &f32) {

	//There could be 2 reasons why an ant is returning back.
	//1) Found food.
	//2) Energy almost depleted.
	//Both follows the strongest pheromone back home. But the 2nd case doesn't drop any pheromone.

	//Following pheromone got a few challenges and things to look out for.
	//1) When following the strongest pheromone, the ants should know at least the last 2 followed pheromones so it doesn't get stuck following the same one.
	//2) Even when following the pheromones, there should be a probability rate where the ant would veer off the path.

	//1st step - find pheromones nearby. Iterate through all the pheromones and calc the distance.
	//2nd step - Filter the ones within range and exclude the visited pheromones.
	//3rd step - Pick the strongest one.
	//4th step - update the ant's movement to that pheromone.
	//5th step - update the visited pheromone array.
	//6th step - if it's the 1st reason, drop pheromone, else do nothing.
	//7th step - Repeat.



	
	let strongest_pheromone: Option<&Pheromone> = find_strongest_pheromone(&ant, pheromones, ant_pheromone_radius);


	if let Some(strongest_pheromone) = strongest_pheromone {
		let direction_to_pheromone: Vec2 =  ant.position - strongest_pheromone.position ;

		if direction_to_pheromone.length() != 0.0 {
			let normalized_direction: Vec2 = direction_to_pheromone/direction_to_pheromone.length(); //normalizing to unit vector.
			ant.moving_direction = normalized_direction;
		}
		

		wandering_ant(ant, world_width, world_height, world_grids, colony_position, true, pheromones,ant_pheromone_radius);

	} else {

		wandering_ant(ant, world_width, world_height, world_grids, colony_position, true, pheromones,ant_pheromone_radius);

	}
	
	return;

}



pub fn move_ant(ant: &mut Ant, world_width: &usize, world_height: &usize, world_grids: &Vec<Vec<Cell>>, colony_position: &Vec2, pheromones: &HashMap<Vec2Key, Pheromone>) -> () {


	let ant_mode = ant.mode;
	let ant_pheromone_radius:f32 = 3.0; //const

	match ant_mode {

		AntMode::Exploring => { wandering_ant(ant, world_width, world_height, world_grids, colony_position, false, pheromones, &ant_pheromone_radius); },
		AntMode::Returning => { wandering_ant(ant, world_width, world_height, world_grids, colony_position, true, pheromones, &ant_pheromone_radius);}

	}

	

}



















