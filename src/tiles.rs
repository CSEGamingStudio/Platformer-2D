//! Implement all the stuff to use Tiled map with amethyst

use amethyst::core::math::Point3;
use amethyst::ecs::prelude::{World, WorldExt};
use amethyst_tiles::Tile;
use tiled::{parse_file, Map};

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
        let map = world.read_resource::<Map>();
        let layer = &map.layers[0];
        let (x, y) = (coordinates[0] as usize, coordinates[1] as usize);
        if layer.tiles[y][x] > 0 {
            Some(layer.tiles[y][x] as usize - 1)
        } else {
            None
        }
    }
}

pub fn load_map(path: &Path) -> Map {
    parse_file(path).unwrap()
}
