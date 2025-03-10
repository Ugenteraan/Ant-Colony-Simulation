mod resources;
mod ui;
mod ecs;

use eframe::egui::Vec2;
use ecs::{component_storage::ComponentStorage, entity_manager::EntityManager};
use ecs::components::{Position};
use resources::{world::World};
use ui::{app::MyApp};


use eframe;


fn main() -> Result<(), eframe::Error> {

    const WIDTH: usize = 600;
    const HEIGHT: usize = 600;
    
    let mut entity_manager: EntityManager = EntityManager::new();
    let ant = entity_manager.create_entity();
    
    let mut component_storage = ComponentStorage::new();

    component_storage.add(ant, Position(Vec2::new(1.0, 1.0))); 
    
    let world: World = World::new(WIDTH, HEIGHT); 
    let options = eframe::NativeOptions::default();
    
    


    eframe::run_native(
        "Ant Colony Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(world)))),
    )
    
}
