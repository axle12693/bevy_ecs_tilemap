mod storage;

use bevy::{
    math::{ UVec2, Vec2 },
    prelude::{ Bundle, Color, Component, Reflect, ReflectComponent },
    render::sync_world::SyncToRenderWorld,
};
pub use storage::*;

use crate::TilemapSize;
use crate::map::TilemapId;

/// A tile position in the tilemap grid.
#[derive(Component, Reflect, Default, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[reflect(Component)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
}

impl TilePos {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Converts a tile position (2D) into an index in a flattened vector (1D), assuming the
    /// tile position lies in a tilemap of the specified size.
    pub fn to_index(&self, tilemap_size: &TilemapSize) -> usize {
        (self.y * tilemap_size.x + self.x) as usize
    }

    /// Checks to see if `self` lies within a tilemap of the specified size.
    pub fn within_map_bounds(&self, map_size: &TilemapSize) -> bool {
        self.x < map_size.x && self.y < map_size.y
    }
}

impl From<TilePos> for UVec2 {
    fn from(pos: TilePos) -> Self {
        UVec2::new(pos.x, pos.y)
    }
}

impl From<&TilePos> for UVec2 {
    fn from(pos: &TilePos) -> Self {
        UVec2::new(pos.x, pos.y)
    }
}

impl From<UVec2> for TilePos {
    fn from(v: UVec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<TilePos> for Vec2 {
    fn from(pos: TilePos) -> Self {
        Vec2::new(pos.x as f32, pos.y as f32)
    }
}

impl From<&TilePos> for Vec2 {
    fn from(pos: &TilePos) -> Self {
        Vec2::new(pos.x as f32, pos.y as f32)
    }
}

/// A texture index into the atlas or texture array for a single tile. Indices in an atlas are horizontal based.
#[derive(Component, Reflect, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[reflect(Component)]
pub struct TileTextureIndex(pub u32);

/// A custom color for the tile.
#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TileColor(pub Color);

impl From<Color> for TileColor {
    fn from(color: Color) -> Self {
        TileColor(color)
    }
}

/// Hides or shows a tile based on the boolean. Default: True
#[derive(Component, Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[reflect(Component)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TileVisible(pub bool);

impl Default for TileVisible {
    fn default() -> Self {
        Self(true)
    }
}

/// Flips the tiles texture along the X, Y or diagonal axes
#[derive(Component, Reflect, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[reflect(Component)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TileFlip {
    /// Flip tile along the x axis.
    pub x: bool,
    /// Flip tile along the Y axis.
    pub y: bool,
    pub d: bool, // anti
}

/// This an optional tile bundle with default components.
#[derive(Bundle, Default, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TileBundle {
    pub position: TilePos,
    pub texture_index: TileTextureIndex,
    pub tilemap_id: TilemapId,
    pub visible: TileVisible,
    pub flip: TileFlip,
    pub color: TileColor,
    pub old_position: TilePosOld,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub sync: SyncToRenderWorld,
}

#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TilePosOld(pub TilePos);

/// A component that is attached to a Tile entity that
/// tells the GPU how to animate the tile.
/// Currently all frames must be aligned in your tilemap.
#[derive(Component, Reflect, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimatedTile {
    /// The start frame index in the tilemap atlas/array (inclusive).
    pub start: u32,
    /// The end frame index in the tilemap atlas/array (exclusive).
    pub end: u32,
    /// The speed the animation plays back at.
    pub speed: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::{ UVec2, Vec2 };

    #[test]
    fn tile_pos_to_index() {
        let map_size = TilemapSize { x: 10, y: 10 };
        let pos = TilePos::new(3, 4);
        assert_eq!(pos.to_index(&map_size), 4 * 10 + 3);
    }

    #[test]
    fn tile_pos_within_bounds() {
        let map_size = TilemapSize { x: 8, y: 8 };
        assert!(TilePos::new(7, 7).within_map_bounds(&map_size));
        assert!(!TilePos::new(8, 0).within_map_bounds(&map_size));
        assert!(!TilePos::new(0, 8).within_map_bounds(&map_size));
    }

    #[test]
    fn conversions_round_trip() {
        let original = TilePos::new(5, 6);

        // TilePos → UVec2 → TilePos
        let as_uvec: UVec2 = original.into();
        assert_eq!(as_uvec, UVec2::new(5, 6));
        let back: TilePos = as_uvec.into();
        assert_eq!(back, original);

        // TilePos → Vec2
        let as_vec: Vec2 = original.into();
        assert_eq!(as_vec, Vec2::new(5.0, 6.0));
    }

    #[test]
    fn visible_default_is_true() {
        assert!(TileVisible::default().0);
    }
}
