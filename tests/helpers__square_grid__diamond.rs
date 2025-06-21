use bevy_ecs_tilemap::helpers::square_grid::neighbors::SquareDirection;
use bevy_ecs_tilemap::helpers::square_grid::diamond::{ DiamondPos };
use bevy_ecs_tilemap::map::TilemapSize;
use bevy_ecs_tilemap::tiles::TilePos;

#[test]
fn as_tile_pos_respects_bounds() {
    let map = TilemapSize { x: 5, y: 5 };
    let inside = DiamondPos::new(3, 4);
    let outside = DiamondPos::new(-1, 0);

    assert_eq!(inside.as_tile_pos(&map), Some(TilePos { x: 3, y: 4 }));
    assert_eq!(outside.as_tile_pos(&map), None);
}

#[test]
fn diamond_offset_follows_square_direction() {
    use SquareDirection::*;
    let map = TilemapSize { x: 4, y: 4 };
    let origin = TilePos { x: 1, y: 1 };

    let up = origin.diamond_offset(&North, &map).unwrap();
    assert_eq!(up, TilePos { x: 1, y: 2 });

    let right = origin.diamond_offset(&East, &map).unwrap();
    assert_eq!(right, TilePos { x: 2, y: 1 });
}
