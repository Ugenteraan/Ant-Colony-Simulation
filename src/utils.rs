
use rand::Rng;
use eframe::egui::Vec2;

pub fn world_to_grid(position: Vec2, cell_size:f32) -> (usize, usize) {
        let x = (position.x / cell_size) as usize;
        let y = (position.y / cell_size) as usize;
        (x, y)
    }


// pub fn gen_rand_direction() -> Vec2 {

//     let mut rng = rand::thread_rng();
//     let angle
// }