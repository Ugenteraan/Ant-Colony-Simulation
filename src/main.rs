
mod simulation;

use simulation::world::World;




fn main() {
    
    const HEIGHT: usize = 30;
    const WIDTH: usize = 50;


    let mut world = World::new(WIDTH, HEIGHT);

    let cell = world.get_cell(4, 4);

    println!("{:?}", cell);



}
