use crate::simulation::world::World;
use crate::gui::renderer;
use eframe::egui;


pub struct MyApp {
    world: World,
}

impl MyApp {
    pub fn new(world: World) -> Self {
        Self {
            world: world,
        }
    }
}

impl eframe::App for MyApp {

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

		egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ant Colony Simulation");

            renderer::draw_world(ui, &self.world);
		});


	}
}