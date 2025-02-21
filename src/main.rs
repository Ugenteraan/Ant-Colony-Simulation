
mod simulation;
mod system;
mod utils;


use bevy::prelude::*;
use glam::Vec2;

use simulation::world::World;
use system::spawn::{spawn_world, spawn_colony};
use system::movements::move_ants;

const HEIGHT: usize = 60;
const WIDTH: usize = 60;
const CELL_SIZE: f32 = 10.0;
const COLONY_POSITION: Vec2 = Vec2::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32);
const DEFAULT_FOOD_IN_COLONY: u32 = 10;


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            (HEIGHT as f32) / 2.0,
            (WIDTH as f32) / 2.0,
            1.0,
        )),
        ..default()
    });
}



fn main() {


    let mut world = World::new(WIDTH, HEIGHT, CELL_SIZE, COLONY_POSITION, DEFAULT_FOOD_IN_COLONY);


    
    App::new()
        .insert_resource(world) // Store world as a resource
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_world)
        .add_systems(Startup, spawn_colony)
        .add_systems(Update, move_ants) // Setup the window and entities
        .run();


}
