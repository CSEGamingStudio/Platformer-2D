#![allow(dead_code)]
#![warn(clippy::all)]

extern crate nalgebra as na;

mod components;
mod game;
mod load;
mod menu;
mod prefabs;
mod systems;
mod tiles;

use amethyst::StateEventReader;
use amethyst::{
    assets::PrefabLoaderSystemDesc,
    audio::AudioBundle,
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
use amethyst_tiles::RenderTiles2D;

use load::LoadState;
use prefabs::PlayerPrefab;
use tiles::MiscTile;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let game_data = GameDataBuilder::new()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderTiles2D::<MiscTile>::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            PrefabLoaderSystemDesc::<PlayerPrefab>::default(),
            "player_loader",
            &[],
        );

    let assets_dir = app_root.join("assets");
    let mut game =
        ApplicationBuilder::<_, _, _, StateEventReader>::new(assets_dir, LoadState::default())?
            .with_frame_limit(FrameRateLimitStrategy::Yield, 24)
            .build(game_data)?;

    game.run();
    Ok(())
}
