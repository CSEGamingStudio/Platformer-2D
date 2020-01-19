//! Load all the assets

use amethyst::{
    assets::{
        Asset, AssetStorage, Format, Handle, Loader, Prefab, PrefabLoader, ProgressCounter,
        RonFormat,
    },
    ecs::prelude::*,
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat},
        Texture,
    },
};

use std::path::Path;

use crate::{game::GameState, prefabs::PlayerPrefab};

#[derive(Default)]
pub struct LoadState {
    progress: ProgressCounter,
    player: Option<Handle<Prefab<PlayerPrefab>>>,
    player_sprite_sheet: Option<Handle<SpriteSheet>>,
    tile_map_sprite_sheet: Option<Handle<SpriteSheet>>,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let player_handle =
            self.load_prefab::<PlayerPrefab, _>(world, &Path::new("player"), RonFormat);
        self.player = Some(player_handle);

        let player_sprite_sheet = self.load_sprite_sheet(world, &Path::new("texture/player"));
        self.player_sprite_sheet = Some(player_sprite_sheet);

        let tile_map_sheet_handle = self.load_sprite_sheet(world, &Path::new("tiles/tileset"));
        self.tile_map_sprite_sheet = Some(tile_map_sheet_handle);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if !self.progress.is_complete() {
            return Trans::None;
        }

        Trans::Switch(Box::new(GameState {
            progress: ProgressCounter::new(),
            player_prefab: self.player.clone(),
            player_sprite_sheet: self.player_sprite_sheet.clone(),
            tile_map_sprite_sheet: self.tile_map_sprite_sheet.clone(),
        }))
    }
}

impl LoadState {
    pub fn load_prefab<T, F>(
        &mut self,
        world: &mut World,
        path: &Path,
        format: F,
    ) -> Handle<Prefab<T>>
    where
        T: Send + Sync + 'static,
        F: Format<<Prefab<T> as Asset>::Data>,
    {
        world.exec(|loader: PrefabLoader<T>| {
            loader.load(
                format!("prefab/{}.ron", path.to_str().unwrap()),
                format,
                &mut self.progress,
            )
        })
    }

    fn load_sprite_sheet(&mut self, world: &mut World, path: &Path) -> Handle<SpriteSheet> {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                format!("{}.png", path.to_str().unwrap()),
                ImageFormat::default(),
                &mut self.progress,
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            format!("{}.ron", path.to_str().unwrap()),
            SpriteSheetFormat(texture_handle),
            &mut self.progress,
            &sprite_sheet_store,
        )
    }
}
