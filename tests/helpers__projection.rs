use bevy_ecs_tilemap::{
    anchor::TilemapAnchor,
    map::{
        HexCoordSystem,
        IsoCoordSystem,
        TilemapGridSize,
        TilemapSize,
        TilemapTileSize,
        TilemapType,
    },
    tiles::TilePos,
};

fn roundtrip(
    original: TilePos,
    map_type: TilemapType,
    grid_size: TilemapGridSize,
    tile_size: TilemapTileSize
) {
    let map_size = TilemapSize { x: 10, y: 10 };
    let anchor = TilemapAnchor::BottomLeft;

    let world = original.center_in_world(&map_size, &grid_size, &tile_size, &map_type, &anchor);
    let recon = TilePos::from_world_pos(
        &world,
        &map_size,
        &grid_size,
        &tile_size,
        &map_type,
        &anchor
    ).expect("round-trip should succeed");

    assert_eq!(original, recon, "round-trip failed for {map_type:?}");
}

#[test]
fn square_roundtrip() {
    roundtrip(
        TilePos { x: 4, y: 6 },
        TilemapType::Square,
        TilemapGridSize { x: 32.0, y: 32.0 },
        TilemapTileSize { x: 32.0, y: 32.0 }
    );
}

#[test]
fn hex_row_even_roundtrip() {
    use HexCoordSystem::*;
    roundtrip(
        TilePos { x: 2, y: 5 },
        TilemapType::Hexagon(RowEven),
        TilemapGridSize { x: 32.0, y: 32.0 },
        TilemapTileSize { x: 32.0, y: 32.0 }
    );
}

#[test]
fn iso_diamond_roundtrip() {
    use IsoCoordSystem::*;
    roundtrip(
        TilePos { x: 1, y: 8 },
        TilemapType::Isometric(Diamond),
        TilemapGridSize { x: 32.0, y: 16.0 },
        TilemapTileSize { x: 32.0, y: 16.0 }
    );
}
