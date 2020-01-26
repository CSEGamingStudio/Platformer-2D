//! The game state

use amethyst::{
    assets::{AssetStorage, Handle, Prefab, ProgressCounter},
    core::{math, Parent, SystemBundle, Time, Transform},
    ecs::prelude::*,
    prelude::*,
    renderer::{
        sprite::{SpriteRender, SpriteSheet},
        Camera,
    },
    utils::application_root_dir,
};
use amethyst_tiles::{Tile, TileMap};
use ncollide2d::shape::{Capsule, Compound, ConvexPolygon, Cuboid, Polyline, Shape, ShapeHandle};
use nphysics2d::{
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    material::{BasicMaterial, MaterialHandle},
    object::{
        BodyPartHandle, BodyStatus, ColliderDesc, DefaultBodySet, DefaultColliderSet, RigidBodyDesc,
    },
    world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
};
use tiled::{Map, ObjectShape};

use std::sync::Arc;

use crate::{
    components::{ColliderComponent, Damageable, Player, RigidBodyComponent, Team},
    prefabs::PlayerPrefab,
    systems::physics::PhysicsBundle,
    tiles::{load_map, MiscTile},
};

#[derive(Clone)]
pub struct Ellipse {
    a: f32, // The first radius
    b: f32, // The second radius
}

impl Ellipse {
    pub fn new(a: f32, b: f32) -> Self {
        Self { a, b }
    }
}

use ncollide2d::{
    bounding_volume::{self, AABB},
    shape::{FeatureId, SupportMap},
};

impl SupportMap<f32> for Ellipse {
    fn support_point(
        &self,
        transform: &na::Isometry2<f32>,
        dir: &na::Vector2<f32>,
    ) -> na::Point2<f32> {
        // Bring `dir` into the ellipse's local frame.
        let local_dir = transform.inverse_transform_vector(dir);

        // Compute the denominator.
        let denom = f32::sqrt(
            local_dir.x * local_dir.x * self.a * self.a
                + local_dir.y * local_dir.y * self.b * self.b,
        );

        // Compute the support point into the ellipse's local frame.
        let local_support_point = na::Point2::new(
            self.a * self.a * local_dir.x / denom,
            self.b * self.b * local_dir.y / denom,
        );

        // Return the support point transformed back into the global frame.
        *transform * local_support_point
    }
}

impl Shape<f32> for Ellipse {
    fn aabb(&self, m: &na::Isometry2<f32>) -> AABB<f32> {
        // Generic method to compute the aabb of a support-mapped shape.
        bounding_volume::support_map_aabb(m, self)
    }

    fn as_support_map(&self) -> Option<&dyn SupportMap<f32>> {
        Some(self)
    }

    fn tangent_cone_contains_dir(
        &self,
        _feature: FeatureId,
        _isometry: &na::Isometry2<f32>,
        _deformation: Option<&[f32]>,
        _dir: &na::Unit<na::Vector2<f32>>,
    ) -> bool {
        false
    }
}

/// The game state
#[derive(Default)]
pub struct GameState<'a, 'b> {
    pub progress: ProgressCounter,
    pub player_prefab: Option<Handle<Prefab<PlayerPrefab>>>,
    pub player_sprite_sheet: Option<Handle<SpriteSheet>>,
    pub tile_map_sprite_sheet: Option<Handle<SpriteSheet>>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for GameState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();
        let bundle = PhysicsBundle;
        bundle.build(world, &mut dispatcher_builder).unwrap();
        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        world.register::<Damageable>();
        world.register::<Team>();
        world.register::<Player>();
        world.register::<RigidBodyComponent>();
        world.register::<ColliderComponent>();

        self.initialise_physics(world);
        let player = self.initialise_player(world, self.player_sprite_sheet.clone().unwrap());
        self.initialise_camera(world, player);
        self.initialise_map(world);

        let mut time = world.write_resource::<Time>();
        time.set_fixed_seconds(1.0 / 60.0);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        _event: StateEvent,
    ) -> SimpleTrans {
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }
}

impl<'a, 'b> GameState<'a, 'b> {
    pub fn new(
        progress: ProgressCounter,
        player_prefab: Option<Handle<Prefab<PlayerPrefab>>>,
        player_sprite_sheet: Option<Handle<SpriteSheet>>,
        tile_map_sprite_sheet: Option<Handle<SpriteSheet>>,
    ) -> Self {
        Self {
            progress,
            player_prefab,
            player_sprite_sheet,
            tile_map_sprite_sheet,
            dispatcher: None,
        }
    }

    fn initialise_physics(&self, world: &mut World) {
        let mut mechanical_world =
            DefaultMechanicalWorld::<f32>::new(na::Vector2::new(0.0, -9.81 * 10.0));
        mechanical_world.set_timestep(1.0 / 60.0);
        let geometrical_world = DefaultGeometricalWorld::<f32>::new();

        world.insert(mechanical_world);
        world.insert(geometrical_world);

        let bodies = DefaultBodySet::<f32>::new();
        let colliders = DefaultColliderSet::<f32>::new();
        let joint_constraints = DefaultJointConstraintSet::<f32>::new();
        let force_generators = DefaultForceGeneratorSet::<f32>::new();

        world.insert(bodies);
        world.insert(colliders);
        world.insert(joint_constraints);
        world.insert(force_generators);
    }

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
        let (half_height, radius) = (9.0, 5.0);

        let rigid_body = RigidBodyDesc::new()
            .translation(na::Vector2::new(pos.0, pos.1))
            .mass(mass)
            .local_center_of_mass(na::Point2::new(radius * 2.0, half_height))
            .kinematic_rotations(true)
            .build();
        let rigid_body_component = RigidBodyComponent::new(rigid_body, world);

        let shape = ShapeHandle::new(Capsule::new(half_height, radius));
        let collider = ColliderDesc::new(shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(
                bounciness, friction,
            )))
            .build(BodyPartHandle(rigid_body_component.handle, 0));
        let collider_component = ColliderComponent::new(collider, world);

        world
            .create_entity()
            .with(Transform::from(math::Vector3::new(
                pos.0 + radius * 2.0,
                pos.1 + half_height,
                0.0,
            )))
            .with(damageable)
            .with(Team::Allies)
            .with(Player)
            .with(sprite_render)
            .with(collider_component)
            .with(rigid_body_component)
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
            math::Vector3::new(width, height, 1),
            math::Vector3::new(tile_width, tile_height, 1),
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
                let transform = Transform::from(math::Vector3::new(
                    (x as f32 * tile_width as f32) - (width as f32 * tile_width as f32) / 2.0,
                    -((y as f32 * tile_height as f32) - (height as f32 * tile_height as f32) / 2.0), // because the y-axis is inverted
                    0.0,
                ));

                let tile = MiscTile.sprite(math::Point3::new(x as u32, y as u32, 0), &world);
                if tile.is_none()
                    || tiles.get(tile.unwrap()).is_none()
                    || tiles[tile.unwrap()].objectgroup.is_none()
                {
                    continue;
                }

                let (rigid_body_component, collider);
                {
                    let mut shapes = Vec::new();
                    for object in &tiles[tile.unwrap()].objectgroup.as_ref().unwrap().objects {
                        let shape: Option<Arc<dyn Shape<f32>>> = match &object.shape {
                            ObjectShape::Rect { width, height } => Some(Arc::new(Cuboid::new(
                                na::Vector2::new(width / 2.0, height / 2.0),
                            ))),
                            ObjectShape::Polygon { points } => Some(Arc::new(
                                ConvexPolygon::try_from_points(
                                    &points
                                        .iter()
                                        .map(|point| na::Point2::new(point.0, point.1))
                                        .collect::<Vec<_>>(),
                                )
                                .unwrap_or_else(|| {
                                    panic!(
                                        "can't build a ConvexHull from the given points : {:#?}",
                                        points
                                    )
                                }),
                            )),
                            ObjectShape::Polyline { points } => Some(Arc::new(Polyline::new(
                                points
                                    .iter()
                                    .map(|point| na::Point2::new(point.0, point.1))
                                    .collect(),
                                None,
                            ))),
                            ObjectShape::Ellipse { width, height } => {
                                Some(Arc::new(Ellipse::new(width / 2.0, height / 2.0)))
                            }
                        };

                        if let Some(shape) = shape {
                            shapes.push((
                                na::Isometry2::translation(object.x, object.y),
                                ShapeHandle::from_arc(shape),
                            ));
                        }
                    }

                    let rigid_body = RigidBodyDesc::new()
                        .translation(na::Vector2::new(
                            (x as f32 * tile_width as f32)
                                - (width as f32 * tile_width as f32) / 2.0,
                            (y as f32 * tile_height as f32)
                                - (height as f32 * tile_height as f32) / 2.0,
                        ))
                        .gravity_enabled(false)
                        .status(BodyStatus::Static)
                        .kinematic_translations(na::Vector2::new(true, true))
                        .kinematic_rotations(true)
                        .build();

                    rigid_body_component = RigidBodyComponent::new(rigid_body, world);

                    collider = if !shapes.is_empty() {
                        let collider = ColliderDesc::new(ShapeHandle::new(Compound::new(shapes)))
                            .material(MaterialHandle::new(BasicMaterial::new(0.0, 1.0)))
                            .build(BodyPartHandle(rigid_body_component.handle, 0));
                        Some(ColliderComponent::new(collider, world))
                    } else {
                        None
                    };
                }

                let mut entity_builder = world.create_entity_unchecked().with(transform);

                if let Some(collider_component) = collider {
                    entity_builder = entity_builder
                        .with(rigid_body_component)
                        .with(collider_component);
                }

                entity_builder.build();
            }
        }
    }
}
