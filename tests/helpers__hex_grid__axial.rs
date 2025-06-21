use bevy_ecs_tilemap::{ prelude::* };
use bevy_ecs_tilemap::helpers::hex_grid::axial::AxialPos;
use bevy_ecs_tilemap::map::{ TilemapGridSize, TilemapSize };

const GRID: TilemapGridSize = TilemapGridSize { x: 32.0, y: 32.0 };

#[test]
fn row_projection_round_trip() {
    let samples = [AxialPos::new(0, 0), AxialPos::new(3, -2), AxialPos::new(-4, 5)];

    for ax in samples {
        let world = ax.center_in_world_row(&GRID);
        let back = AxialPos::from_world_pos_row(&world, &GRID);
        assert_eq!(ax, back, "row-oriented round-trip failed for {ax:?}");
    }
}

#[test]
fn col_projection_round_trip() {
    let samples = [AxialPos::new(0, 0), AxialPos::new(1, 4), AxialPos::new(-3, -2)];

    for ax in samples {
        let world = ax.center_in_world_col(&GRID);
        let back = AxialPos::from_world_pos_col(&world, &GRID);
        assert_eq!(ax, back, "col-oriented round-trip failed for {ax:?}");
    }
}

#[test]
fn tilepos_coord_system_helpers() {
    let map_size = TilemapSize { x: 10, y: 10 };
    let ax = AxialPos::new(3, 2);

    for &sys in &[
        HexCoordSystem::Row,
        HexCoordSystem::Column,
        HexCoordSystem::RowEven,
        HexCoordSystem::RowOdd,
        HexCoordSystem::ColumnEven,
        HexCoordSystem::ColumnOdd,
    ] {
        let tp = ax
            .as_tile_pos_given_coord_system_and_map_size(sys, &map_size)
            .expect("axial pos should be inside map");
        let back = AxialPos::from_tile_pos_given_coord_system(&tp, sys);
        assert_eq!(ax, back, "coord-system conversion failed for {:?} (tile={tp:?})", sys);
    }
}
