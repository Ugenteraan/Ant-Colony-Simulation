use eframe::egui::Vec2;
use rand::Rng;

pub fn gen_rand_direction() -> Vec2 {
    let angle: f32 = rand::rng().random_range(0.0..std::f64::consts::TAU) as f32;
    Vec2::new(angle.cos(), angle.sin())
}

pub fn world_to_grid(position: Vec2) -> (usize, usize) {
    //basically floors the coordinates.
    let x = position.x as usize;
    let y = position.y as usize;

    (x, y)
}
