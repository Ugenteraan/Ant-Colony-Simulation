use glam::Vec2;
use rand::Rng;

pub fn vec2_to_grid_index(position: Vec2, grid_width: usize, grid_height: usize, cell_size: f32) -> (usize, usize) {
    let x = (position.x / cell_size).floor() as isize;
    let y = (position.y / cell_size).floor() as isize;

    // Clamp values to prevent out-of-bounds errors
    let x = x.clamp(0, grid_width as isize - 1) as usize;
    let y = y.clamp(0, grid_height as isize - 1) as usize;

    (x, y)
}


pub fn gen_rand_direction() -> Vec2 {

    let angle: f32 = rand::rng().random_range(0.0..std::f64::consts::TAU) as f32;

    Vec2::new(angle.cos(), angle.sin())
}