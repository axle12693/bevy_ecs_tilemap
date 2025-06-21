use bevy::color::Color;
use bevy_ecs_tilemap::tiles::{ TileBundle, TileFlip, TilePos };

#[test]
fn tile_bundle_defaults_are_consistent() {
    let bundle = TileBundle::default();

    // Position and visibility come from their own tested defaults
    assert_eq!(bundle.position, TilePos::default());
    assert_eq!(bundle.visible.0, true);

    // Old-position starts in sync with `position`
    assert_eq!(bundle.old_position.0, bundle.position);

    // Flip, color and texture index should be zeroed / identity
    assert_eq!(bundle.flip, TileFlip::default());
    assert_eq!(bundle.texture_index.0, 0);
    assert_eq!(bundle.color.0, Color::WHITE);
}
