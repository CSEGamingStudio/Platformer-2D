//! The player component
//! All entity with this component can be controlled by the player

use amethyst::{assets::PrefabData, derive::PrefabData, ecs::prelude::*, Error};
use serde::{Deserialize, Serialize};

/// Basic component for the player, making him move
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Player;

impl Component for Player {
    type Storage = VecStorage<Self>;
}
