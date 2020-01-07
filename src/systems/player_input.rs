//! The system which handle the player input

use amethyst::{
    core::{math::Vector3, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    renderer::sprite::SpriteRender,
};
use amethyst_physics::prelude::*;

use crate::components::{Movable, Player};

#[derive(Default, SystemDesc)]
pub struct PlayerInputSystem;

impl<'s> System<'s> for PlayerInputSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, PhysicsTime>,
        ReadExpect<'s, PhysicsWorld<f32>>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, Movable>,
    );

    fn run(
        &mut self,
        (mut transforms, players, mut sprites, input, time, world, rigid_bodies, movables): Self::SystemData,
    ) {
        for (_transform, _player, sprite, rigid_body, _movable) in (
            &mut transforms,
            &players,
            &mut sprites,
            &rigid_bodies,
            &movables,
        )
            .join()
        {
            let movement = input.axis_value("movement");

            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = time.delta_seconds() * 1_000_000. * mv_amount as f32;
                    sprite.sprite_number = match sprite.sprite_number {
                        0 => 1,
                        1 => 2,
                        2 => 1,
                        _ => unreachable!(),
                    };

                    world
                        .rigid_body_server()
                        .apply_impulse(rigid_body.get(), &Vector3::new(scaled_amount, 0.0, 0.0));
                } else {
                    sprite.sprite_number = 0;
                }
            } else {
                sprite.sprite_number = 0;
            }

            let jump = input.action_is_down("jump").unwrap_or(false);
            let is_on_ground = true;
            if jump && is_on_ground {
                world
                    .rigid_body_server()
                    .apply_impulse(rigid_body.get(), &Vector3::new(0.0, 10_000.0, 0.0));
            }

            let velocity = world.rigid_body_server().linear_velocity(rigid_body.get());
            if velocity.iter().any(|coord| coord.abs() > 100.0) {
                world.rigid_body_server().set_linear_velocity(
                    rigid_body.get(),
                    &Vector3::from_iterator(
                        velocity.iter().map(|coord| coord.max(-100.0).min(100.0)),
                    ),
                )
            }
        }
    }
}
