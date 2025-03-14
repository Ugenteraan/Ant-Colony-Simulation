//main system module.
use crate::ecs::entity_component_storage::EntityComponentStorage;
use crate::ecs::systems::ant_movement;

pub fn system(entity_component_storage: &mut EntityComponentStorage) {
    for entity_component in entity_component_storage.entity_data.iter_mut() {
        //if entity_component.nest.is_some() {}

        if entity_component.ant.is_some() {
            ant_movement::ant_movement(entity_component);
        }
    }
}
