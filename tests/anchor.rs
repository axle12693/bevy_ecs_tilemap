use bevy::prelude::*;
use bevy_ecs_tilemap::{ anchor::*, map::* };
use proptest::prelude::*;

proptest! {
    #[test]
    fn custom_equivalences_hold_across_random_inputs(
        map_x  in 1u32..20,
        map_y  in 1u32..20,
        grid_x in 0.5f32..4.0,
        grid_y in 0.5f32..4.0,  
        tile_x in 0.5f32..4.0,
        tile_y in 0.5f32..4.0,
    ) {
        let map_size  = TilemapSize  { x: map_x, y: map_y };
        let grid_size = TilemapGridSize { x: grid_x, y: grid_y };
        let tile_size = TilemapTileSize { x: tile_x, y: tile_y };
        let map_type  = TilemapType::Square;
        
        // Had to do some trickery because proptest and approx weren't playing nice.
        // Accurate to 3 digits, change as needed.
        let precision = 10f32.powf(3f32);

        // Center
        prop_assert_eq!(
            (TilemapAnchor::Center.as_offset(&map_size, &grid_size, &tile_size, &map_type) * precision).round() / precision,
            (TilemapAnchor::Custom(Vec2::ZERO).as_offset(&map_size, &grid_size, &tile_size, &map_type) * precision).round() / precision
        );

        // Top-left
        prop_assert_eq!(
            (TilemapAnchor::TopLeft.as_offset(&map_size, &grid_size, &tile_size, &map_type) * precision).round() / precision,
            (TilemapAnchor::Custom(Vec2::new(-0.5, 0.5))
                .as_offset(&map_size, &grid_size, &tile_size, &map_type) * precision).round() / precision
        );

        // Bottom-right
        prop_assert_eq!(
            (TilemapAnchor::BottomRight.as_offset(&map_size, &grid_size, &tile_size, &map_type) * precision).round() / precision,
            (TilemapAnchor::Custom(Vec2::new(0.5, -0.5))
                .as_offset(&map_size, &grid_size, &tile_size, &map_type) * precision).round() / precision
        );
    }
}
