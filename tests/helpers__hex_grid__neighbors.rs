use bevy_ecs_tilemap::helpers::hex_grid::neighbors::{HexDirection, HexNeighbors};
use bevy_ecs_tilemap::map::HexCoordSystem;
use bevy_ecs_tilemap::map::TilemapSize;
use bevy_ecs_tilemap::tiles::TilePos;

fn pos(x: u32, y: u32) -> TilePos {
    TilePos { x, y }
}

/// A border tile should yield `None` for neighbors that would fall off the map.
#[test]
fn border_tiles_clamp_neighbors_out_of_bounds() {
    let size = TilemapSize { x: 3, y: 3 };
    // South-west corner
    let corner = pos(0, 2);

    let neighbors =
        HexNeighbors::<TilePos>::get_neighboring_positions(&corner, &size, &HexCoordSystem::Row);

    for dir in [HexDirection::One, HexDirection::Two, HexDirection::Three] {
        assert!(neighbors.get(dir).is_none(), "{dir:?} should be None");
    }

    for dir in [HexDirection::Zero, HexDirection::Four, HexDirection::Five] {
        assert!(neighbors.get(dir).is_some(), "{dir:?} should be Some");
    }
}
