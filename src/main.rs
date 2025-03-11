mod ecs;
mod resources;
mod simulation_setup;
mod ui;
mod utils;

use resources::world::World;
use simulation_setup::Simulation;
use ui::app::MyApp;

use eframe;

fn main() -> Result<(), eframe::Error> {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;

    let mut world: World = World::new(WIDTH, HEIGHT);

    let mut simulation = Simulation::new(&mut world);
    simulation.initialize_entities();
    simulation.run_system();
    simulation.update_world();

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Ant Colony Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(world)))),
    )
}
