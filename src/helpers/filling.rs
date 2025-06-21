use crate::helpers::hex_grid::axial::AxialPos;
use crate::helpers::hex_grid::neighbors::{HEX_DIRECTIONS, HexDirection};
use crate::map::TilemapId;
use crate::prelude::HexCoordSystem;
use crate::tiles::{TileBundle, TileColor, TilePos, TileTextureIndex};
use crate::{TileStorage, TilemapSize};

use bevy::prelude::{Color, Commands};

/// Fills an entire tile storage with the given tile.
pub fn fill_tilemap(
    texture_index: TileTextureIndex,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..size.x {
            for y in 0..size.y {
                let tile_pos = TilePos { x, y };
                let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index,
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });
}

/// Fills a rectangular region with the given tile.
///
/// The rectangular region is defined by an `origin` in [`TilePos`], and a
/// `size` in tiles ([`TilemapSize`]).
pub fn fill_tilemap_rect(
    texture_index: TileTextureIndex,
    origin: TilePos,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..size.x {
            for y in 0..size.y {
                let tile_pos = TilePos {
                    x: origin.x + x,
                    y: origin.y + y,
                };

                let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index,
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });
}

/// Fills a rectangular region with colored versions of the given tile.
///
/// The rectangular region is defined by an `origin` in [`TilePos`], and a
/// `size` in tiles ([`TilemapSize`]).
pub fn fill_tilemap_rect_color(
    texture_index: TileTextureIndex,
    origin: TilePos,
    size: TilemapSize,
    color: Color,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..size.x {
            for y in 0..size.y {
                let tile_pos = TilePos {
                    x: origin.x + x,
                    y: origin.y + y,
                };

                let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index,
                        color: TileColor(color),
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });
}

/// Generates a vector of hex positions that form a ring of given `radius` around the specified
/// `origin`.
///
/// If `radius` is zero, `origin` is the only position in the returned vector.
pub fn generate_hex_ring(origin: AxialPos, radius: u32) -> Vec<AxialPos> {
    if radius == 0 {
        vec![origin]
    } else {
        let mut ring = Vec::with_capacity((radius * 6) as usize);
        let corners = HEX_DIRECTIONS
            .iter()
            .map(|direction| origin + radius * AxialPos::from(direction))
            .collect::<Vec<AxialPos>>();
        // The "tangent" is the direction we must travel in to reach the next corner
        let tangents = (0..6)
            .map(|ix| HexDirection::from(ix + 2).into())
            .collect::<Vec<AxialPos>>();

        for (&corner, &tangent) in corners.iter().zip(tangents.iter()) {
            for k in 0..radius {
                ring.push(corner + k * tangent);
            }
        }

        ring
    }
}

/// Generates a vector of hex positions that form a hexagon of given `radius` around the specified
/// `origin`.
pub fn generate_hexagon(origin: AxialPos, radius: u32) -> Vec<AxialPos> {
    let mut hexagon = Vec::with_capacity(1 + (((6 * radius * (radius + 1)) / 2) as usize));
    for r in 0..radius + 1 {
        hexagon.extend(generate_hex_ring(origin, r));
    }
    hexagon
}

/// Fills a hexagonal region with the given `tile_texture`.
///
/// The rectangular region is defined by an `origin` in [`TilePos`], and a
/// `radius`.
///
/// Tiles that do not fit in the tilemap will not be created.
pub fn fill_tilemap_hexagon(
    texture_index: TileTextureIndex,
    origin: TilePos,
    radius: u32,
    hex_coord_system: HexCoordSystem,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    let tile_positions = generate_hexagon(
        AxialPos::from_tile_pos_given_coord_system(&origin, hex_coord_system),
        radius,
    )
    .into_iter()
    .map(|axial_pos| axial_pos.as_tile_pos_given_coord_system(hex_coord_system))
    .collect::<Vec<TilePos>>();

    commands.entity(tilemap_id.0).with_children(|parent| {
        for tile_pos in tile_positions {
            let tile_entity = parent
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id,
                    texture_index,
                    ..Default::default()
                })
                .id();
            tile_storage.checked_set(&tile_pos, tile_entity);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn axial_distance(a: AxialPos, b: AxialPos) -> u32 {
        let dq = (a.q - b.q).abs() as u32;
        let dr = (a.r - b.r).abs() as u32;
        let ds = (a.q + a.r - (b.q + b.r)).abs() as u32;
        (dq + dr + ds) / 2
    }

    #[test]
    fn ring_radius_zero_is_origin_only() {
        let origin = AxialPos::new(0, 0);
        let ring = generate_hex_ring(origin, 0);
        assert_eq!(ring, vec![origin]);
    }

    #[test]
    fn ring_has_correct_length_and_radius() {
        let origin = AxialPos::new(0, 0);
        for r in 1..=4 {
            let ring = generate_hex_ring(origin, r);
            assert_eq!(ring.len() as u32, r * 6, "radius {r}");
            // no duplicates & all exactly r away
            let uniq: HashSet<_> = ring.iter().cloned().collect();
            assert_eq!(uniq.len(), ring.len(), "radius {r} contains duplicates");
            assert!(ring.iter().all(|p| axial_distance(*p, origin) == r));
        }
    }

    #[test]
    fn hexagon_area_matches_formula_and_contains_rings() {
        let origin = AxialPos::new(0, 0);
        for r in 0..=4 {
            let hex = generate_hexagon(origin, r);
            let expected = 1 + 3 * r * (r + 1);
            assert_eq!(hex.len() as u32, expected, "radius {r}");
            // make sure every inner ring element is inside the hexagon
            for r_inner in 0..=r {
                for p in generate_hex_ring(origin, r_inner) {
                    assert!(hex.contains(&p), "ring {r_inner} not fully inside hex {r}");
                }
            }
        }
    }
}
