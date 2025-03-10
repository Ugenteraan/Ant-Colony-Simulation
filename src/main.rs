mod resources;
mod ui;
mod ecs;
mod utils;
mod simulation_setup;


use simulation_setup::Simulation;
use resources::{world::World};
use ui::{app::MyApp};


use eframe;


fn main() -> Result<(), eframe::Error> {

    const WIDTH: usize = 600;
    const HEIGHT: usize = 600;

    let mut world: World = World::new(WIDTH, HEIGHT);

    let simulation = Simulation::new(&mut world);
    
    let options = eframe::NativeOptions::default();
    
    
    eframe::run_native(
        "Ant Colony Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(world)))),
    )
    
}
