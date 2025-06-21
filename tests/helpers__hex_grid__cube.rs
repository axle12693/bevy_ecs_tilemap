use bevy_ecs_tilemap::helpers::hex_grid::axial::AxialPos;
use bevy_ecs_tilemap::helpers::hex_grid::cube::CubePos;

#[test]
fn axial_round_trip_is_lossless() {
    let axial = AxialPos { q: -5, r: 2 };
    let cube: CubePos = axial.into();
    let back: AxialPos = AxialPos {
        q: cube.q,
        r: cube.r,
    };
    assert_eq!(back, axial, "Axial → Cube → Axial should be identity");
}
