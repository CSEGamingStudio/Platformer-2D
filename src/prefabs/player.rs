//! The player prefab
use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::Entity,
    Error,
};
use serde::{Deserialize, Serialize};

use super::{F32, F32F32};

#[derive(Deserialize, Serialize, PrefabData, Debug)]
#[prefab(PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct PlayerPrefab {
    pub mass: F32,
    pub friction: F32,
    pub bounciness: F32,
    pub pos: F32F32,
}

impl Default for PlayerPrefab {
    fn default() -> Self {
        PlayerPrefab {
            mass: F32(100.0),
            friction: F32(0.95),
            bounciness: F32(0.95),
            pos: F32F32(100.0, 50.0),
        }
    }
}
