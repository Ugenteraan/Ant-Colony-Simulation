use glam::Vec2;


pub struct Colony {
	position: Vec2,
	food_in_store: u32,
}


pub impl Colony {


	fn new(x: f32, y: f32, default_food_in_store: u32) -> Self {

		Colony {
			position: Vec2::new(x, y),
			food_in_store: default_food_in_store,
		}

	}

	// fn produce_ants()
}