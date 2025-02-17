use bevy::prelude::*;
use crate::simulation::world::{World, Cell};
use glam::Vec2;



pub fn spawn_world(mut commands: Commands, world: Res<World>) {

	for y in 0..world.grid.len() { //iterate height-wise.
		for x in 0..world.grid[y].len() { //iterate width-wise.

			let color = match world.grid[y][x] {
				Cell::Colony => Color::srgb(0.8, 0.5, 0.2),
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


// pub fn spawn_colony(mut commands: Commands, mut world: Res<World>) {

// 	let colony_pos = Vec2::new((world.width / 2) as f32 * world.cell_size, (world.height / 2) as f32 * world.cell_size);


// 	commands.spawn((
// 			SpriteBundle {
// 				sprite: Sprite {
					
// 				}
// 			}

// 		))
// }






















