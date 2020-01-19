//! The game state

use amethyst::{
    assets::{AssetStorage, Handle, Prefab, ProgressCounter},
    core::{
        math::{Isometry3, Point3, Translation, Vector3},
        Parent, Time, Transform,
    },
    ecs::prelude::*,
    prelude::*,
    renderer::{
        sprite::{SpriteRender, SpriteSheet},
        Camera,
    },
    utils::application_root_dir,
};
use amethyst_physics::{objects::PhysicsShapeTag, prelude::*};
use amethyst_tiles::{Tile, TileMap};
use tiled::{Map, ObjectShape};

use crate::{
    components::{Damageable, Movable, Player, Team},
    prefabs::PlayerPrefab,
    tiles::{load_map, MiscTile},
};

/// The game state
#[derive(Default)]
pub struct GameState {
    pub progress: ProgressCounter,
    pub player_prefab: Option<Handle<Prefab<PlayerPrefab>>>,
    pub player_sprite_sheet: Option<Handle<SpriteSheet>>,
    pub tile_map_sprite_sheet: Option<Handle<SpriteSheet>>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Damageable>();
        world.register::<Team>();
        world.register::<Player>();
        world.register::<PhysicsHandle<PhysicsShapeTag>>();

        let player = self.initialise_player(world, self.player_sprite_sheet.clone().unwrap());
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
    fn initialise_player(
        &mut self,
        world: &mut World,
        sprite_sheet: Handle<SpriteSheet>,
    ) -> Entity {
        let damageable = Damageable::new(100);
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0,
        };
        let (half_height, radius);
        let shape = {
            half_height = 9.0;
            radius = 5.0;
            let desc = ShapeDesc::Capsule {
                half_height,
                radius,
            };
            let physic_world = world.read_resource::<PhysicsWorld<f32>>();
            physic_world.shape_server().create(&desc)
        };

        let (mass, friction, bounciness, pos) = {
            let mut storage = world.write_resource::<AssetStorage<Prefab<PlayerPrefab>>>();
            let prefab = storage
                .get_mut(&self.player_prefab.clone().unwrap())
                .map(|e| e.entity(0))
                .unwrap()
                .map(|e| e.data())
                .unwrap()
                .unwrap();

            (
                prefab.mass.0,
                prefab.friction.0,
                prefab.bounciness.0,
                prefab.pos,
            )
        };

        let rigid_body = {
            let desc = RigidBodyDesc {
                mode: BodyMode::Dynamic,
                mass,
                bounciness,
                friction,
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
            .with(Transform::from(Vector3::new(pos.0 + radius * 2.0, pos.1 + half_height, pos.2)))
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
        }; */ (800.0, 600.0);

        world
            .create_entity()
            .with(transform)
            .with(Parent::new(entity))
            .with(Camera::standard_2d(width, height))
            .build();
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

        let map_component = TileMap::<MiscTile>::new(
            Vector3::new(width, height, 1),
            Vector3::new(tile_width, tile_height, 1),
            self.tile_map_sprite_sheet.clone(),
        );

        world
            .create_entity()
            .with(Transform::default())
            .with(map_component)
            .build();

        let tiles = &world.read_resource::<Map>().tilesets[0].tiles;

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
                if tile.is_none()
                    || tiles.get(tile.unwrap()).is_none()
                    || tiles[tile.unwrap()].objectgroup.is_none()
                {
                    entity.build();
                    continue;
                }

                {
                    let physic_world = world.read_resource::<PhysicsWorld<f32>>();
                    let mut shapes = Vec::new();
                    for object in &tiles[tile.unwrap()].objectgroup.as_ref().unwrap().objects {
                        let shape = match &object.shape {
                            ObjectShape::Rect { width, height } => Some(ShapeDesc::Cube {
                                half_extents: Vector3::new(width / 2.0, height / 2.0, 0.0),
                            }),
                            ObjectShape::Polygon { points } => {
                                for index in 1..(points.len() - 1) {
                                    let shape = ShapeDesc::Convex {
                                        points: [points[0], points[index], points[index + 1]]
                                            .iter()
                                            .map(|point| Point3::new(point.0, point.1, 0.0))
                                            .collect(),
                                    };
                                    let mut isometry = Isometry3::identity();
                                    isometry.append_translation_mut(&Translation::from(Vector3::new(
                                        x as f32 + object.x,
                                        y as f32 + object.y,
                                        0.0,
                                    )));
                                    shapes.push((isometry, shape));
                                }
                                
                                None
                            },
                            _ => None,
                        };
                        if let Some(shape) = shape {
                            let mut isometry = Isometry3::identity();
                            isometry.append_translation_mut(&Translation::from(Vector3::new(
                                x as f32 + object.x,
                                y as f32 + object.y,
                                0.0,
                            )));
                            shapes.push((isometry, shape));
                        }
                    }
                    if !shapes.is_empty() {
                        let desc = ShapeDesc::Compound { shapes };
                        let shape = physic_world.shape_server().create(&desc);
                        entity = entity.with(shape);
                    }

                    let desc = RigidBodyDesc {
                        mode: BodyMode::Static,
                        bounciness: 0.0,
                        friction: 0.95,
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
                    let rigid_body = physic_world.rigid_body_server().create(&desc);
                    entity = entity.with(rigid_body);
                }

                entity.build();
            }
        }
    }
}
