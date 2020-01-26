pub mod player;

pub use player::PlayerPrefab;

use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PrefabData, PartialEq, Copy, Clone, Component, Debug, Default)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct F32(pub f32);

impl From<f32> for F32 {
    fn from(x: f32) -> Self {
        Self(x)
    }
}

impl From<(f32,)> for F32 {
    fn from(x: (f32,)) -> Self {
        Self(x.0)
    }
}

#[derive(Deserialize, Serialize, PrefabData, PartialEq, Copy, Clone, Component, Debug, Default)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct F32F32(pub f32, pub f32);

impl From<(f32, f32)> for F32F32 {
    fn from(x: (f32, f32)) -> Self {
        Self(x.0, x.1)
    }
}
#[derive(Deserialize, Serialize, PrefabData, PartialEq, Copy, Clone, Component, Debug, Default)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct U32(pub u32);

impl From<u32> for U32 {
    fn from(x: u32) -> Self {
        Self(x)
    }
}

impl From<(u32,)> for U32 {
    fn from(x: (u32,)) -> Self {
        Self(x.0)
    }
}

#[derive(Deserialize, Serialize, PrefabData, PartialEq, Copy, Clone, Component, Debug, Default)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct U32U32(pub u32, pub u32);

impl From<(u32, u32)> for U32U32 {
    fn from(x: (u32, u32)) -> Self {
        Self(x.0, x.1)
    }
}

#[derive(Deserialize, Serialize, PrefabData, PartialEq, Copy, Clone, Component, Debug)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub enum Geometry {
    Void,
    Square,
    Quadrilateral,
}
