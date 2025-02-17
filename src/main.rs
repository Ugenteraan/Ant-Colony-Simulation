
mod simulation;
mod system;
mod utils;


use bevy::prelude::*;
use glam::Vec2;

use simulation::world::World;
use simulation::colony::Colony;
use system::spawn::spawn_world;

const HEIGHT: usize = 20;
const WIDTH: usize = 20;
const CELL_SIZE: f32 = 20.0;
const COLONY_POSITION: Vec2 = Vec2::new((WIDTH / 2) as f32 * CELL_SIZE, (HEIGHT / 2) as f32 * CELL_SIZE);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            (HEIGHT as f32) / 2.0,
            (WIDTH as f32) / 2.0,
            0.0,
        )),
        ..default()
    });
}



fn main() {
    


    // Global counter
    // static NEXT_ID: AtomicU32 = AtomicU32::new(1);


    let mut world = World::new(WIDTH, HEIGHT, CELL_SIZE, COLONY_POSITION);


    
    App::new()
        .insert_resource(world) // Store world as a resource
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_world) // Setup the window and entities
        .run();


}
