
use crate::ecs::entity_data::EntityData;

pub fn ant_movement(entity_component: &mut EntityData) -> () {
    
    if let (Some(direction), Some(velocity)) = (entity_component.direction, entity_component.velocity) {
       entity_component.position.0 += velocity.0 * direction.0;
    }
}
