use bevy_ecs_tilemap::{map::TilemapSize, tiles::TilePos, *};
use helpers::square_grid::neighbors::SquareDirection;

#[test]
fn square_offset_roundtrip() {
    let map_size = TilemapSize { x: 5, y: 5 };
    let origin = TilePos::new(2, 2);

    // Take a step north then south – we should land back on origin.
    let north = origin
        .square_offset(&SquareDirection::North, &map_size)
        .expect("in-bounds north neighbour");
    let back = north
        .square_offset(&SquareDirection::South, &map_size)
        .expect("in-bounds south neighbour");

    assert_eq!(back, origin);
}

#[test]
fn square_offset_out_of_bounds_returns_none() {
    let map_size = TilemapSize { x: 4, y: 4 };
    let edge = TilePos::new(0, 0);

    assert!(
        edge.square_offset(&SquareDirection::South, &map_size)
            .is_none()
    );
    assert!(
        edge.square_offset(&SquareDirection::West, &map_size)
            .is_none()
    );
}
