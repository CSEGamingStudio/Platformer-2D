use amethyst::ecs::{Component, VecStorage, World, WorldExt};
use nphysics2d::object::*;

pub struct RigidBodyComponent {
    pub handle: DefaultBodyHandle,
}

impl RigidBodyComponent {
    pub fn new(rigid_body: RigidBody<f32>, world: &World) -> Self {
        let mut body_set = world.write_resource::<DefaultBodySet<f32>>();
        let handle = body_set.insert(rigid_body);
        Self { handle }
    }
}

impl Component for RigidBodyComponent {
    type Storage = VecStorage<Self>;
}

pub struct ColliderComponent {
    pub handle: DefaultColliderHandle,
}

impl ColliderComponent {
    pub fn new(rigid_body: Collider<f32, DefaultColliderHandle>, world: &World) -> Self {
        let mut collider_set = world.write_resource::<DefaultColliderSet<f32>>();
        let handle = collider_set.insert(rigid_body);
        Self { handle }
    }
}

impl Component for ColliderComponent {
    type Storage = VecStorage<Self>;
}
