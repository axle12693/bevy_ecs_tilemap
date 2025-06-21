use bevy_ecs_tilemap::{
    helpers::square_grid::{ neighbors::SquareDirection::*, staggered::StaggeredPos },
    map::{ TilemapGridSize, TilemapSize },
    tiles::TilePos,
};

#[test]
fn as_tile_pos_bounds() {
    let map = TilemapSize { x: 10, y: 10 };

    let inside = StaggeredPos::new(5, 5);
    let outside = StaggeredPos::new(-1, 0);

    assert_eq!(inside.as_tile_pos(&map).unwrap(), TilePos { x: 5, y: 5 });
    assert!(outside.as_tile_pos(&map).is_none());
}

#[test]
fn tilepos_staggered_offset() {
    let map = TilemapSize { x: 3, y: 3 };

    let origin = TilePos { x: 1, y: 1 };
    assert_eq!(origin.staggered_offset(&North, &map).unwrap(), TilePos { x: 1, y: 2 });

    // Edge should return None
    let edge = TilePos { x: 0, y: 0 };
    assert!(edge.staggered_offset(&West, &map).is_none());
}

#[test]
fn world_roundtrip() {
    let grid = TilemapGridSize { x: 32.0, y: 32.0 };
    let original = StaggeredPos::new(4, 2);

    let world = original.center_in_world(&grid);
    let round = StaggeredPos::from_world_pos(&world, &grid);

    assert_eq!(original, round);
}
