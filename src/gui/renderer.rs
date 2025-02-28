use eframe::egui;
use crate::simulation::world::World;




pub fn draw_world(ui: &mut egui::Ui, world: &World, colony_texture: &Option<egui::TextureHandle>, ant_texture: &Option<egui::TextureHandle>, food_texture: &Option<egui::TextureHandle>, available_size: &egui::Vec2, painter: &egui::Painter) -> () {

	
	draw_ant(ui, world, ant_texture, available_size, painter);
	draw_colony(ui, world, colony_texture, available_size, painter);
	draw_food(ui, world, food_texture, available_size, painter);
	
	
}



fn draw_colony(ui: &mut egui::Ui, world: &World, colony_texture: &Option<egui::TextureHandle>, available_size: &egui::Vec2, painter: &egui::Painter) -> () {

	

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


fn draw_ant(ui: &mut egui::Ui, world: &World, ant_texture: &Option<egui::TextureHandle>, available_size: &egui::Vec2, painter: &egui::Painter) -> () {


	let ant_icon_size_x = available_size.x / 60.0;
	let ant_icon_size_y = available_size.y / 60.0;


	for ant in world.colony.ants.iter() {

		
		let ui_x = ant.position.x * (available_size.x/world.width as f32);
		let ui_y = ant.position.y * (available_size.y/world.height as f32);

		let ui_pos = egui::Pos2::new(ui_x, ui_y);


		if let Some(texture) = ant_texture {

			ui.put(
				egui::Rect::from_min_size(ui_pos, egui::Vec2::new(ant_icon_size_x, ant_icon_size_y)),
				egui::Image::new(texture)
					.fit_to_exact_size(egui::Vec2::new(ant_icon_size_x, ant_icon_size_y)),
			);

		} else{
			painter.circle_filled(ui_pos, 5.0, egui::Color32::from_rgb(255, 255, 255));
		}
	}
}


fn draw_food(ui: &mut egui::Ui, world: &World, food_texture: &Option<egui::TextureHandle>, available_size: &egui::Vec2, painter: &egui::Painter) -> () {

	let food_icon_size_x = available_size.x / 50.0;
	let food_icon_size_y = available_size.y / 50.0;

	for food in world.foods.iter() {

		let ui_x = food.position.x * (available_size.x/world.width as f32);
		let ui_y = food.position.y * (available_size.y/world.height as f32);

		let ui_pos = egui::Pos2::new(ui_x, ui_y);

		if let Some(texture) = food_texture {
			ui.put(
				egui::Rect::from_min_size(ui_pos, egui::Vec2::new(food_icon_size_x, food_icon_size_y)),
				egui::Image::new(texture)
					.fit_to_exact_size(egui::Vec2::new(food_icon_size_x, food_icon_size_y)),
			);
		} else{
			painter.circle_filled(ui_pos, 5.0, egui::Color32::from_rgb(255, 0, 255));
		}
	}


}










