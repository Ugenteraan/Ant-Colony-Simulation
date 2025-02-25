
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
pub fn change_direction(border: bool, blocked: bool) -> (f32, f32) {

    //if the ant is blocked by something (like another ant), 
    //turn the ant by 45 degrees.
    if blocked {
        let angle: f32 = 0.785398; //45 degrees
        return (angle.cos(), angle.sin());
    }

    //if it's a border collision, then changethe direction to a value between 150 degrees to 210 degrees.
    if border {
        let angle: f32 = rand::rng().random_range(2.61799..3.66519) as f32;
        return (angle.cos(), angle.sin());
    }

    
    //the angle generated is in between -90 degrees to 90 degrees.
    let angle: f32 = rand::rng().random_range(-1.0*std::f64::consts::PI/2.0..std::f64::consts::PI/2.0) as f32;
    return (angle.cos(), angle.sin());
}
