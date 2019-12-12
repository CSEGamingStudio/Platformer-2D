//! The game state

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    core::{math::Vector3, Parent, Transform},
    ecs::prelude::*,
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        Camera, Texture,
    },
    utils::application_root_dir,
    window::ScreenDimensions,
};
use amethyst_tiles::TileMap;

use std::path::Path;

use crate::{
    components::{Damageable, Player, Team},
    tiles::{load_map, MiscTile},
};

/// The game state
#[derive(Default)]
pub struct GameState {
    progress: ProgressCounter,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let player_sprite_sheet = self.load_sprite_sheet(world, "texture/player");

        world.register::<Damageable>();
        world.register::<Team>();
        world.register::<Player>();

        let player = self.initialise_player(world, player_sprite_sheet);
        self.initialise_camera(world, &player);
        self.intialise_map(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        Trans::None
    }
}

impl GameState {
    fn initialise_player(&self, world: &mut World, sprite_sheet: Handle<SpriteSheet>) -> Entity {
        let damageable = Damageable::new(100);
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(Transform::default())
            .with(damageable)
            .with(Team::Allies)
            .with(Player)
            .with(sprite_render)
            .build()
    }

    fn initialise_camera(&self, world: &mut World, entity: &Entity) {
        let mut transform = Transform::default();
        transform.set_translation_z(1.0);

        let (width, height) = /* {
            println!("before");
            let dim = world.read_resource::<ScreenDimensions>();
            println!("after");
            (dim.width(), dim.height())
        }; */ (800., 600.);

        world
            .create_entity()
            .with(transform)
            .with(Parent::new(*entity))
            .with(Camera::standard_2d(width, height))
            .build();
    }

    fn load_sprite_sheet(&self, world: &mut World, path: &str) -> Handle<SpriteSheet> {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                format!("{}.png", path),
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            format!("{}.ron", path),
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        )
    }

    fn intialise_map(&self, world: &mut World) {
        let path =  &*application_root_dir()
            .unwrap()
            .join("assets")
            .join("tiles")
            .join("map.tmx");
        
        let map = load_map(path);

        let width = map.width;
        let height = map.height;
        let tile_width = map.tile_width;
        let tile_height = map.tile_height;

        world.insert(map);
        
        let sprite_sheet_handle = self.load_sprite_sheet(world, "tiles/tileset");

        let map_component = TileMap::<MiscTile>::new(
            Vector3::new(width, height, 1),
            Vector3::new(tile_width, tile_height, 1),
            Some(sprite_sheet_handle)
        );
        
        world
            .create_entity()
            .with(Transform::default())
            .with(map_component)
            .build();
    }
}
