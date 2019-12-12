//! The system which handle the player input

use amethyst::{
    core::{Time, Transform},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    renderer::sprite::SpriteRender,
};

use crate::components::Player;

pub struct PlayerInputSystem;

impl<'s> System<'s> for PlayerInputSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, players, mut sprites, input, time): Self::SystemData) {
        for (transform, player, sprite) in (&mut transforms, &players, &mut sprites).join() {
            let movement = input.axis_value("movement");

            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = time.fixed_seconds() * 100. * mv_amount as f32;
                    sprite.sprite_number = match sprite.sprite_number {
                        0 => 1,
                        1 => 2,
                        2 => 1,
                        _ => unreachable!()
                    };
                    transform.prepend_translation_x(scaled_amount);
                } else {
                    sprite.sprite_number = 0;
                }
            } else {
                sprite.sprite_number = 0;
            }
        }
    }
}
