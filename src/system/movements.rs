// use bevy::prelude::*;
// use crate::simulation::ant::Ant; 



// /// Updates ant movement every frame
// pub fn move_ants(mut query: Query<&mut Transform, With<Ant>>) {

// 	println!("{:?}", query);
//     for mut transform in query.iter_mut() {
//     	println!("Here");
//     	println!("{:?}", transform);
//         transform.translation.x += 1.0; // ✅ Move ants to the right
//     }

// }