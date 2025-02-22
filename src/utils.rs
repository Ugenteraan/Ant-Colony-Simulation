
use rand::Rng;
use eframe::egui::Vec2;

pub fn world_to_grid(position: Vec2, cell_size:f32) -> (usize, usize) {
    let x = (position.x / cell_size) as usize;
    let y = (position.y / cell_size) as usize;
    (x, y)
}


pub fn gen_rand_direction() -> Vec2 {

    let angle: f32 = rand::rng().random_range(0.0..std::f64::consts::TAU) as f32;
    Vec2::new(angle.cos(), angle.sin())
}