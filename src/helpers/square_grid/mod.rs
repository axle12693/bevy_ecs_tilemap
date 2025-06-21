pub mod diamond;
pub mod neighbors;
pub mod staggered;

use crate::helpers::square_grid::diamond::DiamondPos;
use crate::helpers::square_grid::neighbors::{SQUARE_OFFSETS, SquareDirection};
use crate::helpers::square_grid::staggered::StaggeredPos;
use crate::tiles::TilePos;
use crate::{TilemapGridSize, TilemapSize};
use bevy::math::Vec2;
use std::ops::{Add, Mul, Sub};

/// Position for tiles arranged in a square coordinate system.
///
/// It is vector-like. In other words: it makes sense to add and subtract
/// two `SquarePos`, and it makes sense to multiply a `SquarePos` by
/// an [`i32`] scalar.
///
/// A `SquarePos` can be mapped to world space, and a world space position can be mapped to
/// the tile with `SquarePos` containing said world space position.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SquarePos {
    pub x: i32,
    pub y: i32,
}

impl Add<SquarePos> for SquarePos {
    type Output = SquarePos;

    fn add(self, rhs: SquarePos) -> Self::Output {
        SquarePos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<SquarePos> for SquarePos {
    type Output = SquarePos;

    fn sub(self, rhs: SquarePos) -> Self::Output {
        SquarePos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<SquarePos> for i32 {
    type Output = SquarePos;

    fn mul(self, rhs: SquarePos) -> Self::Output {
        SquarePos {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl From<&TilePos> for SquarePos {
    #[inline]
    fn from(tile_pos: &TilePos) -> Self {
        Self {
            x: tile_pos.x as i32,
            y: tile_pos.y as i32,
        }
    }
}

impl From<&DiamondPos> for SquarePos {
    #[inline]
    fn from(diamond_pos: &DiamondPos) -> Self {
        let DiamondPos { x, y } = *diamond_pos;
        SquarePos { x, y }
    }
}

impl From<DiamondPos> for SquarePos {
    #[inline]
    fn from(diamond_pos: DiamondPos) -> Self {
        let DiamondPos { x, y } = diamond_pos;
        SquarePos { x, y }
    }
}

impl From<&StaggeredPos> for SquarePos {
    #[inline]
    fn from(staggered_pos: &StaggeredPos) -> Self {
        let StaggeredPos { x, y } = *staggered_pos;
        SquarePos { x, y: y + x }
    }
}

impl From<StaggeredPos> for SquarePos {
    #[inline]
    fn from(staggered_pos: StaggeredPos) -> Self {
        let StaggeredPos { x, y } = staggered_pos;
        SquarePos { x, y: y + x }
    }
}

impl SquarePos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Project a vector representing a fractional tile position (i.e. the components can be `f32`)
    /// into world space.
    ///
    /// This is a helper function for
    /// [`center_in_world`](Self::center_in_world),
    /// [`corner_offset_in_world`](Self::corner_offset_in_world) and
    /// [`corner_in_world`](Self::corner_in_world).
    #[inline]
    pub fn project(pos: Vec2, grid_size: &TilemapGridSize) -> Vec2 {
        Vec2::new(grid_size.x * pos.x, grid_size.y * pos.y)
    }

    /// Returns the position of this tile's center, in world space.
    #[inline]
    pub fn center_in_world(&self, grid_size: &TilemapGridSize) -> Vec2 {
        Self::project(Vec2::new(self.x as f32, self.y as f32), grid_size)
    }

    /// Returns the offset to the corner of a tile in the specified `corner_direction`,
    /// in world space
    #[inline]
    pub fn corner_offset_in_world(
        corner_direction: SquareDirection,
        grid_size: &TilemapGridSize,
    ) -> Vec2 {
        let corner_offset = SquarePos::from(corner_direction);
        let corner_pos = 0.5 * Vec2::new(corner_offset.x as f32, corner_offset.y as f32);
        Self::project(corner_pos, grid_size)
    }

    /// Returns the coordinate of the corner of a tile in the specified `corner_direction`,
    /// in world space
    #[inline]
    pub fn corner_in_world(
        &self,
        corner_direction: SquareDirection,
        grid_size: &TilemapGridSize,
    ) -> Vec2 {
        let center = Vec2::new(self.x as f32, self.y as f32);

        let corner_offset = SquarePos::from(corner_direction);
        let corner_pos = 0.5 * Vec2::new(corner_offset.x as f32, corner_offset.y as f32);

        Self::project(center + corner_pos, grid_size)
    }

    /// Returns the tile containing the given world position.
    #[inline]
    pub fn from_world_pos(world_pos: &Vec2, grid_size: &TilemapGridSize) -> SquarePos {
        let normalized_world_pos = Vec2::new(world_pos.x / grid_size.x, world_pos.y / grid_size.y);
        let Vec2 { x, y } = normalized_world_pos;
        SquarePos {
            x: (x + 0.5).floor() as i32,
            y: (y + 0.5).floor() as i32,
        }
    }

    /// Try converting into a [`TilePos`].
    ///
    /// Returns `None` if either one of `self.x` or `self.y` is negative, or lies outside of the
    /// bounds of `map_size`.
    #[inline]
    pub fn as_tile_pos(&self, map_size: &TilemapSize) -> Option<TilePos> {
        TilePos::from_i32_pair(self.x, self.y, map_size)
    }

    /// Calculate offset in the given direction.
    #[inline]
    pub fn offset(&self, direction: &SquareDirection) -> SquarePos {
        *self + SQUARE_OFFSETS[*direction as usize]
    }
}

impl TilePos {
    /// Get the neighbor lying in the specified direction from this position, if it  fits on the map
    /// and assuming that this is a map using the standard (non-isometric) square coordinate system
    #[inline]
    pub fn square_offset(
        &self,
        direction: &SquareDirection,
        map_size: &TilemapSize,
    ) -> Option<TilePos> {
        SquarePos::from(self)
            .offset(direction)
            .as_tile_pos(map_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::square_grid::neighbors::SquareDirection;
    use crate::{TilemapGridSize, TilemapSize, tiles::TilePos};

    fn gs() -> TilemapGridSize {
        TilemapGridSize { x: 32.0, y: 32.0 }
    }

    #[test]
    fn add_sub_mul_work() {
        let a = SquarePos::new(2, -3);
        let b = SquarePos::new(-4, 8);

        assert_eq!(a + b, SquarePos::new(-2, 5));
        assert_eq!(a - b, SquarePos::new(6, -11));
        assert_eq!(3 * a, SquarePos::new(6, -9));
    }

    #[test]
    fn conversions_are_correct() {
        let tp = TilePos::new(4, 7);
        assert_eq!(SquarePos::from(&tp), SquarePos::new(4, 7));

        let dp = DiamondPos { x: -2, y: 5 };
        assert_eq!(SquarePos::from(dp), SquarePos::new(-2, 5));

        let sp = StaggeredPos { x: 3, y: 1 };
        assert_eq!(SquarePos::from(sp), SquarePos::new(3, 4));
    }

    #[test]
    fn project_and_unproject_roundtrip() {
        let pos = SquarePos::new(5, 9);
        let world = pos.center_in_world(&gs());
        assert_eq!(SquarePos::from_world_pos(&world, &gs()), pos);
    }

    #[test]
    fn corners_in_world_match_offset_helpers() {
        let pos = SquarePos::new(0, 0);
        for dir in [
            SquareDirection::NorthWest,
            SquareDirection::NorthEast,
            SquareDirection::SouthWest,
            SquareDirection::SouthEast,
        ] {
            let by_center = pos.corner_in_world(dir, &gs());
            let by_offset =
                pos.center_in_world(&gs()) + SquarePos::corner_offset_in_world(dir, &gs());
            assert_eq!(by_center, by_offset);
        }
    }

    #[test]
    fn as_tile_pos_respects_bounds() {
        let map_size = TilemapSize { x: 10, y: 10 };
        let inside = SquarePos::new(3, 9);
        assert_eq!(inside.as_tile_pos(&map_size), Some(TilePos::new(3, 9)));

        let neg = SquarePos::new(-1, 0);
        let out = SquarePos::new(10, 0);
        assert!(neg.as_tile_pos(&map_size).is_none());
        assert!(out.as_tile_pos(&map_size).is_none());
    }

    #[test]
    fn offset_matches_square_offsets_table() {
        let origin = SquarePos::new(0, 0);
        for dir in [
            SquareDirection::North,
            SquareDirection::East,
            SquareDirection::South,
            SquareDirection::West,
        ] {
            assert_eq!(origin.offset(&dir), SQUARE_OFFSETS[dir as usize]);
        }
    }
}
