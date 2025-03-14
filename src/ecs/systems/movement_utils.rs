use crate::utils;
mod config;

pub fn check_border_collision(new_position: Vec2) -> bool {
    let (x, y) = utils::world_to_grid(new_position);

    //check if the position goes beyond or below the border.
    if x >= config::WORLD_WIDTH - 1 || y >= config::WORLD_HEIGHT - 1 || x <= 0 || y <= 0 {
        return true;
    }

    return false;
}
