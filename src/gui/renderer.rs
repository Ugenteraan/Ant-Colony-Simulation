use eframe::egui;
use crate::simulation::world::World;


pub fn draw_world(ui: &mut egui::Ui, world: &World, colony_texture: &Option<egui::TextureHandle>){

	let available_size = ui.available_size();

	let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());



	let ui_x = world.colony.position.x * (available_size.x/world.width as f32);
	let ui_y = world.colony.position.y * (available_size.y/world.height as f32);
	
	let colony_icon_size_x = available_size.x / 40.0;
	let colony_icon_size_y = available_size.y / 40.0;

	let ui_pos = egui::Pos2::new(ui_x, ui_y);

	if let Some(texture) = colony_texture {
		ui.put(
			//the second argument here should be the one resizing the image. However, we seem to need the
			//.fit_exact_size in order to do the resizing. And the code doesn't work without this first line below.
			egui::Rect::from_min_size(ui_pos, egui::Vec2::new(colony_icon_size_x, colony_icon_size_y)), 
			egui::Image::new(texture)
            	.fit_to_exact_size(egui::Vec2::new(colony_icon_size_x, colony_icon_size_y)),
		);
	} else{
		painter.circle_filled(ui_pos, 10.0, egui::Color32::from_rgb(255, 255, 255));
	}
}