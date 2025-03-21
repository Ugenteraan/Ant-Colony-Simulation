
mod simulation;
mod gui;
mod utils;



use eframe::egui::Vec2;
use simulation::world::World;
use gui::app;




fn main() -> Result<(), eframe::Error> {

    const HEIGHT: usize = 100;
    const WIDTH: usize = 100;
    const COLONY_POSITION: Vec2 = Vec2::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32);
    const DEFAULT_FOOD_IN_COLONY: u32 = 20;


    let world = World::new(WIDTH, HEIGHT, COLONY_POSITION, DEFAULT_FOOD_IN_COLONY);

    let options = eframe::NativeOptions::default();

    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default()
    //         .with_decorations(false) // Hide the OS-specific "chrome" around the window
    //         .with_inner_size([WIDTH as f32, HEIGHT as f32])
    //         .with_min_inner_size([WIDTH as f32, HEIGHT as f32])
    //         .with_transparent(true), // To have rounded corners we need transparency

    //     ..Default::default()
    // };

    eframe::run_native(
        "Ant Colony Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(app::MyApp::new(world)))), 
    )
}

