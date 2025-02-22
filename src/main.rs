
mod simulation;
mod gui;
mod utils;



use eframe::egui::Vec2;
use simulation::world::World;
use gui::app;




fn main() -> Result<(), eframe::Error> {

    const HEIGHT: usize = 100;
    const WIDTH: usize = 100;
    const CELL_SIZE: f32 = 50.0;
    const COLONY_POSITION: Vec2 = Vec2::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32);
    const DEFAULT_FOOD_IN_COLONY: u32 = 10;


    let world = World::new(WIDTH, HEIGHT, CELL_SIZE, COLONY_POSITION, DEFAULT_FOOD_IN_COLONY);

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Ant Colony Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(app::MyApp::new(world)))), 
    )
}

