use crate::map::TilemapType;
use crate::tiles::TilePos;
use crate::{TilemapAnchor, TilemapGridSize, TilemapSize, TilemapTileSize, Transform};

// Deprecated. Skipping tests.

/// Calculates a [`Transform`] for a tilemap that places it so that its center is at
/// `(0.0, 0.0, z)` in world space.
#[deprecated(since = "0.15.1", note = "please use `TilemapAnchor::Center` instead")]
pub fn get_tilemap_center_transform(
    map_size: &TilemapSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    z: f32,
) -> Transform {
    let tile_size = TilemapTileSize::new(grid_size.x, grid_size.y);
    let low = TilePos::new(0, 0).center_in_world(
        map_size,
        grid_size,
        &tile_size,
        map_type,
        &TilemapAnchor::None,
    );
    let high = TilePos::new(map_size.x - 1, map_size.y - 1).center_in_world(
        map_size,
        grid_size,
        &tile_size,
        map_type,
        &TilemapAnchor::None,
    );

    let diff = high - low;

    Transform::from_xyz(-diff.x / 2.0, -diff.y / 2.0, z)
}
