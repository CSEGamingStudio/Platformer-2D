//! The damageable component
//! All the entities that could receive damage should have it

use amethyst::ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Component for entities that have a life and can be damaged
#[derive(Deserialize, Serialize)]
/*#[prefab(Component)]*/
#[serde(deny_unknown_fields)]
pub struct Damageable {
    life: i32,
    max_life: i32,
}

impl Component for Damageable {
    type Storage = DenseVecStorage<Self>;
}

impl Damageable {
    pub fn new(life: i32) -> Self {
        Self {
            life,
            max_life: life,
        }
    }

    pub fn life(&self) -> i32 {
        self.life
    }

    pub fn max_life(&self) -> i32 {
        self.max_life
    }

    pub fn is_alive(&self) -> bool {
        self.life != 0
    }

    pub fn heal(&mut self, amount: i32) {
        self.life = (self.life + amount).max(self.max_life);
    }

    pub fn damage(&mut self, amount: i32) {
        self.life = (self.life - amount).min(0);
    }
}
