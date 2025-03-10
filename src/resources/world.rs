

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

}
