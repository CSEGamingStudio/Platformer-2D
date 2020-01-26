//! The system which handle the player input

use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    renderer::sprite::SpriteRender,
};
use nphysics2d::object::DefaultBodySet;

use crate::components::{Player, RigidBodyComponent};

#[derive(Default, SystemDesc)]
pub struct PlayerInputSystem;

impl<'s> System<'s> for PlayerInputSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<StringBindings>>,
        WriteExpect<'s, DefaultBodySet<f32>>,
        ReadStorage<'s, RigidBodyComponent>,
    );

    fn run(&mut self, (players, mut sprites, input, mut body_set, rigid_bodies): Self::SystemData) {
        for (_player, sprite, rigid_body) in (&players, &mut sprites, &rigid_bodies).join() {
            let movement = input.axis_value("movement");
            let body = body_set.rigid_body_mut(rigid_body.handle).unwrap();

            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = 10.0 * mv_amount as f32;
                    sprite.sprite_number = match sprite.sprite_number {
                        0 => 1,
                        1 => 2,
                        2 => 1,
                        _ => unreachable!(),
                    };

                    let velocity = body.velocity().linear + na::Vector2::new(scaled_amount, 0.0);
                    body.set_linear_velocity(velocity);
                } else {
                    sprite.sprite_number = 0;
                }
            } else {
                sprite.sprite_number = 0;
            }

            let jump = input.action_is_down("jump").unwrap_or(false);
            let is_on_ground = true;
            if jump && is_on_ground {
                let velocity = body.velocity().linear + na::Vector2::new(0.0, 10.0);
                body.set_linear_velocity(velocity);
            }
        }
    }
}
