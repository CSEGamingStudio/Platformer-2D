pub mod damageable;
pub mod movable;
pub mod physics;
pub mod player;
pub mod team;

pub use self::{
    damageable::Damageable,
    movable::Movable,
    physics::{ColliderComponent, RigidBodyComponent},
    player::Player,
    team::Team,
};
