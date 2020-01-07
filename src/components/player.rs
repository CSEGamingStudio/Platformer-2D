//! The player component
//! All entities with this component can be controlled by the player

use amethyst::ecs::prelude::{Component, NullStorage};

/// Basic component for the player, making him movable by the user
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}
