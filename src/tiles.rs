//! Implement all the stuff to use Tiled map with amethyst

use amethyst::{
    core::{
        math::{Point3, Vector3},
        Parent, Transform,
    },
    ecs::prelude::{Join, World, WorldExt},
    renderer::Camera,
    tiles::{CoordinateEncoder, DrawTiles2DBounds, Map, Region, Tile, TileMap},
    window::ScreenDimensions,
};
use tiled::{parse_file, Map as TiledMap};

use std::path::Path;

/// The tile used by the amethyst engine
#[derive(Clone, Copy, Debug)]
pub struct MiscTile;

impl Default for MiscTile {
    fn default() -> Self {
        MiscTile
    }
}

impl Tile for MiscTile {
    fn sprite(&self, coordinates: Point3<u32>, world: &World) -> Option<usize> {
        assert_eq!(coordinates[2], 0);
        let map = world.read_resource::<TiledMap>();
        let tileset = map.get_tileset_by_gid(1).unwrap();
        let tiles = &map
            .layers
            .iter()
            .filter(|layer| layer.name == "tiles")
            .nth(0)
            .expect("Error : no layer with name `tiles`")
            .tiles;
        let (y, x) = (coordinates[0] as usize, coordinates[1] as usize);

        if *tiles
            .get(x)?
            .get(y)?
            > 0
        {
            Some(tiles[x][y] as usize - 1)
        } else {
            None
        }
    }
}

pub fn load_map(path: &Path) -> TiledMap {
    parse_file(path).unwrap()
}

#[derive(Debug)]
pub struct DrawTilesBounds;

impl DrawTiles2DBounds for DrawTilesBounds {
    fn bounds<T: Tile, E: CoordinateEncoder>(map: &TileMap<T, E>, world: &World) -> Region {
        let (map_width, map_height) = (map.dimensions().x - 1, map.dimensions().y - 1);
        let (tile_width, tile_height) = (map.tile_dimensions().x, map.tile_dimensions().y);
        let (width, height) = {
            let screen_dimensions = world.read_resource::<ScreenDimensions>();
            (
                screen_dimensions.width() as u32 / tile_width,
                screen_dimensions.height() as u32 / tile_height,
            )
        };
        let cameras = world.read_storage::<Camera>();
        let transforms = world.read_storage::<Transform>();
        let parents = world.read_storage::<Parent>();

        let entity = (&cameras, &transforms, &parents).join().nth(0);
        return Region::new(Point3::new(0, 0, 0), Point3::new(map_width as u32, map_height as u32, 0));
        if entity.is_none() {
            return Region::new(
                Point3::new(0, 0, 0),
                Point3::new(width as u32, height as u32, 0),
            );
        }
        let origin = Vector3::new(map_width as i32 / 2, map_height as i32 / 2, -1);

        let (_, transform, parent) = entity.unwrap();

        let parent_translation = *transforms.get(parent.entity).unwrap().translation();

        let translation =
            (Vector3::from_vec(transform.translation().iter().copied().collect::<Vec<_>>())
                + origin.map(|coord| coord as f32)
                - parent_translation)
                .map(|coord| coord as i32);
        let top_left =
            Point3::from(translation + Vector3::new(-(width as i32), -(height as i32), 0));
        let bottom_right = Point3::from(translation + Vector3::new(width as i32, height as i32, 0));
        Region::new(
            Point3::from_slice(
                &top_left
                    .iter()
                    .take(3)
                    .map(|x| x - 5) // load a bit more than requested
                    .map(|x| x.max(0) as u32) // force to have a positive number
                    .collect::<Vec<_>>(),
            ),
            Point3::from_slice(
                &bottom_right
                    .iter()
                    .take(3)
                    .map(|x| x + 5) // idem
                    .map(|x| x.max(0) as u32) // idem
                    .collect::<Vec<_>>(),
            ),
        )
    }
}
