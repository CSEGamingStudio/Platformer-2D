//! The movable component
//! All entities with this component can be moved by the physics

use amethyst::ecs::{Component, NullStorage};

// A component that permitted the entity to be affect by physics
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Movable;

impl Component for Movable {
    type Storage = NullStorage<Self>;
}
