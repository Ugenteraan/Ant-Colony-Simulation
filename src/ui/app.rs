use crate::resources::world::{Cell, World};
use crate::ui::renderer;
use eframe::egui;
use image::ImageReader;
use std::io::Cursor;

pub struct MyApp {
    world: World,
    nest_texture: Option<egui::TextureHandle>,
    ant_texture: Option<egui::TextureHandle>,
}

impl MyApp {
    pub fn new(world: World) -> Self {
        Self {
            world: world,
            nest_texture: None,
            ant_texture: None,
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
                egui::ColorImage::from_rgba_unmultiplied([w as _, h as _], &pixels),
                egui::TextureOptions::default(),
            ));
        }
    }

    pub fn load_nest_texture(&mut self, ctx: &egui::Context) {
        if self.nest_texture.is_none() {
            let image_data = include_bytes!("../../assets/anthill.png");
            let image = ImageReader::new(Cursor::new(image_data))
                .with_guessed_format()
                .expect("Failed to read image!")
                .decode()
                .expect("Failed to decode image!")
                .to_rgba8();

            let (w, h) = image.dimensions();
            let pixels = image.into_raw();

            self.nest_texture = Some(ctx.load_texture(
                "nest_icon",
                egui::ColorImage::from_rgba_unmultiplied([w as _, h as _], &pixels),
                egui::TextureOptions::default(),
            ));
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_nest_texture(ctx);
        self.load_ant_texture(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ant Colony Simulaton");

            let available_size = ui.available_size();
            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());

            for (y, row) in self.world.grid.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    match cell {
                        Cell::Nest => renderer::draw_nest(
                            ui,
                            &self.world,
                            x,
                            y,
                            &self.nest_texture,
                            &available_size,
                            &painter,
                        ),
                        Cell::Ant => renderer::draw_ant(
                            ui,
                            &self.world,
                            x,
                            y,
                            &self.ant_texture,
                            &available_size,
                            &painter,
                        ),
                        _ => {}
                    }
                }
            }
        });
    }
}
