use eframe::egui;
use crate::simulation::world::World;


pub fn draw_world(ui: &mut egui::Ui, world: &World){

	let available_size = ui.available_size();

    let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());

    
    let center_x = available_size.x / 2.0;
    let center_y = available_size.y / 2.0;

    let ui_x = world.colony.position.x * (available_size.x/world.width as f32);
    let ui_y = world.colony.position.y * (available_size.y/world.height as f32);

    // Convert world position to UI coordinates
    let ui_pos = egui::Pos2::new(ui_x, ui_y);
    
    // Draw the colony as a small brown circle
    painter.circle_filled(ui_pos, 10.0, egui::Color32::from_rgb(255, 255, 255));
}