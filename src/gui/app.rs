use crate::simulation::{world::World, system::System};
use crate::gui::renderer;
use eframe::egui;
use image::ImageReader as ImageReader;
use std::io::Cursor;
use std::time::{Instant, Duration};


const ANT_ENERGY: f32 = 50.0;
const ANT_LIFESPAN: u32 = 500;
const ANT_SPEED: f32 = 0.3;
const ANT_TURN_RATE: f32 = 0.01;

pub struct MyApp {
    world: World,
    system: System,
    colony_texture: Option<egui::TextureHandle>,
    ant_texture: Option<egui::TextureHandle>,
    last_update: Instant,
}

impl MyApp {
    pub fn new(world: World) -> Self {

    	let system = System::new(ANT_ENERGY, ANT_LIFESPAN, ANT_SPEED, ANT_TURN_RATE); //initialize the system that governs the simulation.

        Self {
            world: world,
            system: system,
            colony_texture: None,
            ant_texture: None,
            last_update: Instant::now()
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


    pub fn load_ant_texture(&mut self, ctx: &egui::Context) {

    	if self.ant_texture.is_none() {
    		let image_data = include_bytes!("../../assets/ant.png"); 
    		let image = ImageReader::new(Cursor::new(image_data))
    			.with_guessed_format()
    			.expect("Failed to read image!")
    			.decode()
    			.expect("Failed to decode image!")
    			.to_rgba8();

    		let (w, h) = image.dimensions();
    		let pixels = image.into_raw();

    		self.ant_texture = Some(ctx.load_texture(
    			"ant_icon",
    			egui::ColorImage::from_rgba_unmultiplied([w as _, h as _,], &pixels),
    			egui::TextureOptions::default(),
    		));
    	}
    }



}

impl eframe::App for MyApp {

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

		let now = Instant::now();
        let elapsed = now - self.last_update;


        ctx.request_repaint(); //repaint the GUI at every call.
		self.load_colony_texture(ctx);
		self.load_ant_texture(ctx);

		egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ant Colony Simulation");

            let available_size = ui.available_size();

			let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());

			renderer::draw_world(ui, &self.world, &self.colony_texture, &self.ant_texture, &available_size, &painter);

			//update the world at interval to avoid system crash.
			if elapsed > Duration::from_millis(32) { 

	            self.system.update_world(&mut self.world); //update the simulation's system.
	            self.last_update = now;
        	}
            
		});

		

	}
}