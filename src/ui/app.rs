
use crate::resources::{world::World};
use eframe::egui;


pub struct MyApp {
    world: World
}


impl MyApp {
    
    pub fn new(world: World) -> Self {
    
        Self {
            world: world
        }

    }

}


impl eframe::App for MyApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
                                                ui.heading("Ant Colony Simulaton");

                                                //let available_size = ui.available_size();
                                                //let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());

                                                });
    }

}
