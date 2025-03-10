
use crate::resources::world::{World, Cell};
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

            let available_size = ui.available_size();
            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());

            
            for (y, row) in self.world.grid.iter().enumerate() {

                for (x, cell) in row.iter().enumerate() {
                    
                    match cell {
                        Cell::Nest => {painter.circle_filled(egui::Pos2::new(x as f32, y as f32), 10.0, egui::Color32::from_rgb(255, 255, 255));},
                        _ => {}
                    }

                }
            }
        });
    }

}
