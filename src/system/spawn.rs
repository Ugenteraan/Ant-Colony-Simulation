use bevy::prelude::*;
use bevy::ui::widget::ImageNode; 

use crate::simulation::world::{World, Cell};
use crate::simulation::ant::Ant; 
use crate::utils;
use glam::Vec2;



pub fn spawn_world(mut commands: Commands, world: Res<World>) {

	for y in 0..world.grid.len() { //iterate height-wise.
		for x in 0..world.grid[y].len() { //iterate width-wise.

			let color = match world.grid[y][x] {
				// Cell::Colony => Color::srgb(0.8, 0.5, 0.2),
				Cell::Food => Color::srgb(0.0, 1.0, 0.0),
				Cell::Pheromone(_) => Color::srgb(0.0, 0.0, 1.0),
				_ => continue,
			};

			let position: Vec3 = Vec3::new(x as f32 * world.cell_size, y as f32 * world.cell_size, 0.0);

			commands.spawn((
					SpriteBundle {
						sprite: Sprite {
							color, 
							custom_size: Some(Vec2::new(world.cell_size, world.cell_size)),
							..default()
						},
						transform: Transform::from_translation(position),
						..default()
					}

				));

		}
	}

}


pub fn spawn_colony(mut commands: Commands, asset_server: Res<AssetServer>, mut world: Res<World>) {

	let colony_pos = world.colony.position;

	let colony_texture: Handle<Image> = asset_server.load("anthill.png");
	let world_height: usize = world.height;
	let world_width: usize = world.width;


	commands.spawn(

		SpriteBundle {
			sprite: Sprite {
		        image: colony_texture, 
		        custom_size: Some(Vec2::new((world_width as f32)/4.0, (world_height as f32)/4.0)),
		        ..default()
		    	},
		    transform: Transform::from_translation(Vec3::new(colony_pos.x, colony_pos.y, 0.0)),
		    ..default()
	   } 
    );

}


pub fn spawn_ants(mut commands: Commands, asset_server: Res<AssetServer>, mut world: ResMut<World>) {

	const ANT_ENERGY: f32 = 50.0;
	const ANT_LIFESPAN: u32 = 500;

	let colony_pos = world.colony.position;

	let ant_texture: Handle<Image> = asset_server.load("ant.png");
	

	for i in 0..world.colony.food_in_colony {

		let random_direction: Vec2 = utils::gen_rand_direction();
		let initial_position: Vec2 = colony_pos + random_direction; //update the ant's first move.

		world.insert_ant(initial_position, random_direction, ANT_ENERGY, ANT_LIFESPAN);

		
	}




}
























