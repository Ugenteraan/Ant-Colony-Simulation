use crate::simulation::world::World;
use crate::gui::renderer;
use eframe::egui;
use image::io::Reader as ImageReader;
use std::io::Cursor;


pub struct MyApp {
    world: World,
    colony_texture: Option<egui::TextureHandle>,
}

impl MyApp {
    pub fn new(world: World) -> Self {
        Self {
            world: world,
            colony_texture: None
        }
    }


    pub fn load_colony_texture(&mut self, ctx: &egui::Context) {

    	if self.colony_texture.is_none() {
    		let image_data = include_bytes!("../../assets/anthill.png"); 
    		let image = ImageReader::new(Cursor::new(image_data))
    			.with_guessed_format()
    			.expect("Failed to read image!")
    			.decode()
    			.expect("Failed to decode image!")
    			.to_rgba8();

    		let (w, h) = image.dimensions();
    		let pixels = image.into_raw();

    		self.colony_texture = Some(ctx.load_texture(
    			"colony_icon",
    			egui::ColorImage::from_rgba_unmultiplied([w as _, h as _,], &pixels),
    			egui::TextureOptions::default(),
    		));
    	}
    }
}

impl eframe::App for MyApp {

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

		self.load_colony_texture(ctx);

		egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ant Colony Simulation");

            renderer::draw_world(ui, &self.world, &self.colony_texture);
		});


	}
}