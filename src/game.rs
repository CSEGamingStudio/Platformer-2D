//! The game state

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    core::{
        math::{Point3, Vector3},
        Parent, Time, Transform,
    },
    ecs::prelude::*,
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        Camera, Texture,
    },
    utils::application_root_dir,
};
use amethyst_physics::{objects::PhysicsShapeTag, prelude::*};
use amethyst_tiles::{Tile, TileMap};
use tiled::Map;

use std::path::Path;

use crate::{
    components::{Damageable, Movable, Player, Team},
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
        let player_sprite_sheet = self.load_sprite_sheet(world, &Path::new("texture/player"));

        world.register::<Damageable>();
        world.register::<Team>();
        world.register::<Player>();
        world.register::<PhysicsHandle<PhysicsShapeTag>>();

        let player = self.initialise_player(world, player_sprite_sheet);
        self.initialise_camera(world, player);
        self.initialise_map(world);

        let mut time = world.write_resource::<Time>();
        time.set_fixed_seconds(1. / 60.);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        _event: StateEvent,
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
        let shape = {
            let desc = ShapeDesc::Capsule {
                half_height: 20.,
                radius: 10.,
            };
            let physic_world = world.read_resource::<PhysicsWorld<f32>>();
            physic_world.shape_server().create(&desc)
        };

        let rigid_body = {
            let desc = RigidBodyDesc {
                mode: BodyMode::Dynamic,
                mass: 50.0,
                bounciness: 0.0,
                friction: 100.0,
                belong_to: vec![CollisionGroup::new(0)],
                collide_with: vec![CollisionGroup::new(0)],
                contacts_to_report: 3,
                lock_translation_z: true,
                lock_rotation_x: true,
                lock_rotation_y: true,
                lock_rotation_z: true,
                ..Default::default()
            };
            let physic_world = world.read_resource::<PhysicsWorld<f32>>();
            physic_world.rigid_body_server().create(&desc)
        };

        world
            .create_entity()
            .with(Transform::from(Vector3::new(0.0, 200.0, 0.0)))
            .with(damageable)
            .with(Team::Allies)
            .with(Player)
            .with(sprite_render)
            .with(shape)
            .with(rigid_body)
            .with(Movable)
            .build()
    }

    fn initialise_camera(&self, world: &mut World, entity: Entity) {
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
            .with(Parent::new(entity))
            .with(Camera::standard_2d(width, height))
            .build();
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

    fn initialise_map(&mut self, world: &mut World) {
        let path = &*application_root_dir()
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

        for (x, row) in world.read_resource::<Map>().layers[0]
            .tiles
            .iter()
            .enumerate()
        {
            for (y, _column) in row.iter().enumerate() {
                let transform = Transform::from(Vector3::new(
                    (x as f32 * tile_width as f32) - (width as f32 * tile_width as f32) / 2.0,
                    -((y as f32 * tile_height as f32) - (height as f32 * tile_height as f32) / 2.0), // because the y-axis is inverted
                    0.0,
                ));

                let mut entity = world.create_entity_unchecked().with(transform);

                let tile = MiscTile.sprite(Point3::new(x as u32, y as u32, 0), &world);

                entity = match tile {
                    Some(0..=7) => {
                        let shape = {
                            let desc = ShapeDesc::Cube {
                                half_extents: Vector3::new(
                                    tile_width as f32 / 2.0,
                                    tile_height as f32 / 2.0,
                                    0.0,
                                ),
                            };
                            let physic_world = world.read_resource::<PhysicsWorld<f32>>();
                            physic_world.shape_server().create(&desc)
                        };

                        let rigid_body = {
                            let desc = RigidBodyDesc {
                                mode: BodyMode::Static,
                                bounciness: 0.0,
                                friction: 1.0,
                                belong_to: vec![CollisionGroup::new(0)],
                                collide_with: vec![CollisionGroup::new(0)],
                                lock_translation_x: true,
                                lock_translation_y: true,
                                lock_translation_z: true,
                                lock_rotation_x: true,
                                lock_rotation_y: true,
                                lock_rotation_z: true,
                                ..Default::default()
                            };
                            let physic_world = world.read_resource::<PhysicsWorld<f32>>();
                            physic_world.rigid_body_server().create(&desc)
                        };

                        entity.with(shape).with(rigid_body)
                    }
                    Some(8) => entity,
                    Some(_) => entity,
                    None => entity,
                };

                entity.build();
            }
        }

        let sprite_sheet_handle = self.load_sprite_sheet(world, &Path::new("tiles/tileset"));

        let map_component = TileMap::<MiscTile>::new(
            Vector3::new(width, height, 1),
            Vector3::new(tile_width, tile_height, 1),
            Some(sprite_sheet_handle),
        );

        world
            .create_entity()
            .with(Transform::default())
            .with(map_component)
            .build();
    }
}
