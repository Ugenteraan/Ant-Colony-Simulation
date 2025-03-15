// List all the components here with their fields.

use eframe::egui::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position(pub Vec2);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity(pub f32);

//can be used to represent pheromone's intensity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intensity(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Direction(pub Vec2);

//the chance for an entity to change its direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TurningChance(pub f32);

//can be used for food to represent its capacity.
//can be used for nest to represent the food stored capacity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Capacity(pub f32);

//mode of the ants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntMode {
    Exploring,
    Returning,
}
