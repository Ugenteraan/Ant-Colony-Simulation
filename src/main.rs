
mod simulation;
// mod system;
mod utils;

use eframe::egui::Vec2;
use eframe::egui;

use simulation::world::World;
// use system::spawn::{spawn_world, spawn_colony};
// use system::movements::move_ants;

const HEIGHT: usize = 60;
const WIDTH: usize = 60;
const CELL_SIZE: f32 = 10.0;
const COLONY_POSITION: Vec2 = Vec2::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32);
const DEFAULT_FOOD_IN_COLONY: u32 = 10;


struct MyApp {
    world: World,
}

impl MyApp {
    fn new() -> Self {
        Self {
            world: World::new(WIDTH, HEIGHT, CELL_SIZE, COLONY_POSITION, DEFAULT_FOOD_IN_COLONY),
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ant Colony Simulation");
            
            ui.label(format!(
                "Colony Position: ({:.1}, {:.1})",
                self.world.colony.position.x, self.world.colony.position.y
            ));

            let (response, painter) = ui.allocate_painter(egui::Vec2::new(500.0, 500.0), egui::Sense::hover());

            let available_size = ui.available_size();
            let center_x = available_size.x / 2.0;
            let center_y = available_size.y / 2.0;

            // Convert world position to UI coordinates
            let ui_pos = egui::Pos2::new(self.world.colony.position.x, self.world.colony.position.y);
            
            // Draw the colony as a small brown circle
            painter.circle_filled(ui_pos, 100.0, egui::Color32::from_rgb(255, 255, 255));
        });
    }
}


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    // let world = World::new(WIDTH, HEIGHT, CELL_SIZE, COLONY_POSITION, DEFAULT_FOOD_IN_COLONY); // ✅ Initialize world

    eframe::run_native(
        "Ant Colony Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))), // ✅ Pass `world` into `MyApp`
    )
}

