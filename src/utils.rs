
use rand::Rng;
use eframe::egui::Vec2;

pub fn world_to_grid(position: Vec2) -> (usize, usize) {
    let x = position.x as usize; //basically, just floors the value.
    let y = position.y as usize;
    (x, y)
}

//when spawning the ants.
pub fn gen_rand_direction() -> Vec2 {

    let angle: f32 = rand::rng().random_range(0.0..std::f64::consts::TAU) as f32;
    Vec2::new(angle.cos(), angle.sin())
}

//changing currently moving ants.
pub fn change_direction(border: bool) -> Vec2 {

    if !border {
        //the angle would not be more than 90 degrees.
        let angle: f32 = rand::rng().random_range(0.0..std::f64::consts::PI/2.0) as f32;
        return Vec2::new(angle.cos(), angle.sin());
    } else {
        let angle: f32 = rand::rng().random_range(std::f64::consts::PI/2.0..std::f64::consts::TAU*0.75) as f32;
        return Vec2::new(angle.cos(), angle.sin());
    }
}
