use crate::resources::world::World;
use eframe::egui;

pub fn draw_nest(
    ui: &mut egui::Ui,
    world: &World,
    nest_position_x: usize,
    nest_position_y: usize,
    nest_texture: &Option<egui::TextureHandle>,
    available_size: &egui::Vec2,
    painter: &egui::Painter,
) -> () {
    let ui_x = nest_position_x as f32 * (available_size.x / world.width as f32) - 10.0;
    let ui_y = nest_position_y as f32 * (available_size.y / world.height as f32) - 10.0;

    let nest_icon_size_x = available_size.x / 30.0;
    let nest_icon_size_y = available_size.y / 30.0;

    let ui_pos = egui::Pos2::new(ui_x, ui_y);

    if let Some(texture) = nest_texture {
        ui.put(
            //the second argument here should be the one resizing the image. However, we seem to need the
            //.fit_exact_size in order to do the resizing. And the code doesn't work without this first line below.
            egui::Rect::from_min_size(ui_pos, egui::Vec2::new(nest_icon_size_x, nest_icon_size_y)),
            egui::Image::new(texture)
                .fit_to_exact_size(egui::Vec2::new(nest_icon_size_x, nest_icon_size_y)),
        );
    } else {
        painter.circle_filled(ui_pos, 10.0, egui::Color32::from_rgb(255, 255, 255));
    }
}

pub fn draw_ant(
    ui: &mut egui::Ui,
    world: &World,
    x: usize,
    y: usize,
    ant_texture: &Option<egui::TextureHandle>,
    available_size: &egui::Vec2,
    painter: &egui::Painter,
) -> () {
    let ant_icon_size_x = available_size.x / 60.0;
    let ant_icon_size_y = available_size.y / 60.0;

    // -5.0 to center the ant's UI body.
    let ui_x = x as f32 * (available_size.x / world.width as f32) - 5.0;
    let ui_y = y as f32 * (available_size.y / world.height as f32) - 5.0;

    let ui_pos = egui::Pos2::new(ui_x, ui_y);

    if let Some(texture) = ant_texture {
        ui.put(
            egui::Rect::from_min_size(ui_pos, egui::Vec2::new(ant_icon_size_x, ant_icon_size_y)),
            egui::Image::new(texture)
                .fit_to_exact_size(egui::Vec2::new(ant_icon_size_x, ant_icon_size_y)),
        );
    } else {
        painter.circle_filled(ui_pos, 5.0, egui::Color32::from_rgb(255, 255, 255));
    }
}
