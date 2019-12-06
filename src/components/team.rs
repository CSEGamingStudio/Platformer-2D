//! The team component
//! Entities of the same team shouldn't attack themselves

use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::Transform,
    derive::PrefabData,
    ecs::prelude::*,
    Error,
};
use serde::{Deserialize, Serialize};

/// A team : member of the same team shouldn't hit them
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub enum Team {
    Allies,
    Enemies,
}

impl Component for Team {
    type Storage = DenseVecStorage<Self>;
}
