
use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::ecs::entity_manager::Entity;


pub trait Component: Any {}

impl<T: Any> Component for T {} //blanket implementation to implement the Component trait to any
                                //types.

pub struct ComponentStorage {
    components: HashMap<Entity, Box<dyn Component>>
}


impl ComponentStorage {
    
    pub fn new() -> Self {
        ComponentStorage {
            components: HashMap::new()
        }
    }

    //'static keyword is used for safety so that no component with a reference field allowed.
    pub fn add<T: Component + 'static> (&mut self, entity:Entity, component: T) {
        
        self.components.insert(entity, Box::new(component));
    }

}


