#![allow(unused_variables)]

extern crate amethyst;
extern crate amethyst_tiles;
extern crate amethyst_window;
extern crate tiled;

mod components;
mod game;
mod menu;
mod prefabs;
mod systems;
mod tiles;

use amethyst::StateEventReader;
use amethyst::{
    audio::{AudioBundle, DjSystem},
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
use amethyst_tiles::{MortonEncoder2D, RenderTiles2D};

use game::GameState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let world = World::new();
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with(
            systems::PlayerInputSystem,
            "player_input_system",
            &["input_system"],
        );

    let assets_dir = app_root.join("assets");
    let mut game: CoreApplication<'_, GameData<'static, 'static>, StateEvent, StateEventReader> =
        ApplicationBuilder::new(assets_dir, GameState::default())?
            .with_frame_limit(FrameRateLimitStrategy::Yield, 24)
            .build(game_data)?;
    game.run();
    Ok(())
}
