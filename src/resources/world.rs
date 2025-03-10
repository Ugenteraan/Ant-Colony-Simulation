
use crate::utils;

use eframe::egui::Vec2;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Empty, 
    Ant, 
    Nest,
    Food,
}


#[derive(Clone, Debug)]
pub struct World {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,
}


impl World {

    pub fn new(width: usize, height: usize) -> Self {
        
        World {
            width: width,
            height: height,
            grid: vec![vec![Cell::Empty; width]; height],
        }
    }

    

    pub fn set_grid(&mut self, position: Vec2, cell: Cell) -> bool {
        //attempts to set the grid with a specific cell. If success, sends true, else false. 
        let (x, y) = utils::world_to_grid(position);
        
        //only set the cell if it's empty. 
        match self.grid[x][y] {
            Cell::Empty => {
                            self.grid[x][y] = cell;
                            return true;
            },
            _ => {return false;}
        }

    }

}
