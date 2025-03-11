use eframe::egui::Vec2;

pub fn world_to_grid(position: Vec2) -> (usize, usize) {
    //basically floors the coordinates.
    let x = position.x as usize;
    let y = position.y as usize;

    (x, y)
}
