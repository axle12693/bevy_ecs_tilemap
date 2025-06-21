use bevy_ecs_tilemap::{
    helpers::square_grid::{ neighbors::{ Neighbors, SquareDirection }, staggered::StaggeredPos },
    map::TilemapSize,
    tiles::{ TilePos, TileStorage },
};

#[test]
fn square_neighboring_positions_centre_of_small_map() {
    let map = TilemapSize { x: 3, y: 3 };
    let centre = TilePos::new(1, 1);

    let neighbors = Neighbors::get_square_neighboring_positions(&centre, &map, true);

    // we should get all eight neighbouring cells
    for (dx, dy) in [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ] {
        let expected = TilePos::new((1isize + dx) as u32, (1isize + dy) as u32);
        assert!(
            neighbors.iter().any(|p| *p == expected),
            "expected to find neighbour at {expected:?}"
        );
    }
}

#[test]
fn square_neighboring_positions_edges_clamp_to_none() {
    let map = TilemapSize { x: 2, y: 2 };
    let corner = TilePos::new(0, 0);

    let neighbors = Neighbors::get_square_neighboring_positions(&corner, &map, true);

    // (0,0) only has East, North, NorthEast inside the map
    assert_eq!(neighbors.east, Some(TilePos::new(1, 0)));
    assert_eq!(neighbors.north, Some(TilePos::new(0, 1)));
    assert_eq!(neighbors.north_east, Some(TilePos::new(1, 1)));

    assert!(neighbors.north_west.is_none());
    assert!(neighbors.west.is_none());
    assert!(neighbors.south_west.is_none());
    assert!(neighbors.south.is_none());
    assert!(neighbors.south_east.is_none());
}

#[test]
fn staggered_neighboring_positions_respects_offset() {
    let map: TilemapSize = TilemapSize { x: 3, y: 3 };
    let start: TilePos = TilePos::new(1, 1);

    let neighbors: Neighbors<TilePos> = Neighbors::get_staggered_neighboring_positions(
        &start,
        &map,
        false
    );

    // only cardinals requested
    assert!(neighbors.north_east.is_none());
    assert!(neighbors.south_west.is_none());
    assert_eq!(
        neighbors.north,
        Some(StaggeredPos::from(&start).offset(&SquareDirection::North).as_tile_pos(&map).unwrap())
    );
    assert_eq!(
        neighbors.east,
        Some(StaggeredPos::from(&start).offset(&SquareDirection::East).as_tile_pos(&map).unwrap())
    );
}

#[test]
fn tile_storage_entity_lookup() {
    // Smoke-test the `entities` helper.
    use bevy::prelude::Entity;

    let map = TilemapSize { x: 2, y: 2 };
    let mut storage = TileStorage::empty(map);

    let a = Entity::from_raw(1);
    let b = Entity::from_raw(2);
    storage.set(&TilePos::new(1, 0), a);
    storage.set(&TilePos::new(0, 1), b);

    let pos = TilePos::new(0, 0);
    let neighbors = Neighbors::get_square_neighboring_positions(&pos, &map, false);
    let entity_neighbors = neighbors.entities(&storage);

    assert_eq!(entity_neighbors.east, Some(a));
    assert_eq!(entity_neighbors.north, Some(b));
    // other cardinals None, diagonals not requested
    assert!(entity_neighbors.south.is_none());
    assert!(entity_neighbors.west.is_none());
}
