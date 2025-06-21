use bevy::{
    ecs::{
        entity::{EntityMapper, MapEntities},
        reflect::ReflectMapEntities,
    },
    prelude::*,
};

use crate::map::TilemapSize;

use super::TilePos;

/// Used to store tile entities for fast look up.
/// Tile entities are stored in a grid. The grid is always filled with None.
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component, MapEntities)]
pub struct TileStorage {
    tiles: Vec<Option<Entity>>,
    pub size: TilemapSize,
}

impl MapEntities for TileStorage {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for entity in self.tiles.iter_mut().flatten() {
            *entity = entity_mapper.get_mapped(*entity);
        }
    }
}

impl TileStorage {
    /// Creates a new tile storage that is empty.
    pub fn empty(size: TilemapSize) -> Self {
        Self {
            tiles: vec![None; size.count()],
            size,
        }
    }

    /// Gets a tile entity for the given tile position, if an entity is associated with that tile
    /// position.
    ///
    /// Panics if the given `tile_pos` doesn't lie within the extents of the underlying tile map.
    pub fn get(&self, tile_pos: &TilePos) -> Option<Entity> {
        self.tiles[tile_pos.to_index(&self.size)]
    }

    /// Gets a tile entity for the given tile position, if:
    /// 1) the tile position lies within the underlying tile map's extents *and*
    /// 2) there is an entity associated with that tile position;
    ///
    /// otherwise it returns `None`.
    pub fn checked_get(&self, tile_pos: &TilePos) -> Option<Entity> {
        if tile_pos.within_map_bounds(&self.size) {
            self.tiles[tile_pos.to_index(&self.size)]
        } else {
            None
        }
    }

    /// Sets a tile entity for the given tile position.
    ///
    /// If there is an entity already at that position, it will be replaced.
    ///
    /// Panics if the given `tile_pos` doesn't lie within the extents of the underlying tile map.
    pub fn set(&mut self, tile_pos: &TilePos, tile_entity: Entity) {
        self.tiles[tile_pos.to_index(&self.size)].replace(tile_entity);
    }

    /// Sets a tile entity for the given tile position, if the tile position lies within the
    /// underlying tile map's extents.
    ///
    /// If there is an entity already at that position, it will be replaced.
    pub fn checked_set(&mut self, tile_pos: &TilePos, tile_entity: Entity) {
        if tile_pos.within_map_bounds(&self.size) {
            self.tiles[tile_pos.to_index(&self.size)].replace(tile_entity);
        }
    }

    /// Returns an iterator with all of the positions in the grid.
    pub fn iter(&self) -> impl Iterator<Item = &Option<Entity>> {
        self.tiles.iter()
    }

    /// Returns mutable iterator with all of the positions in the grid.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<Entity>> {
        self.tiles.iter_mut()
    }

    /// Removes any stored `Entity` at the given tile position, leaving `None` in its place and
    /// returning the `Entity`.
    ///
    /// Panics if the given `tile_pos` doesn't lie within the extents of the underlying tile map.
    pub fn remove(&mut self, tile_pos: &TilePos) -> Option<Entity> {
        self.tiles[tile_pos.to_index(&self.size)].take()
    }

    /// Remove any stored `Entity` at the given tile position, leaving `None` in its place and
    /// returning the `Entity`.
    ///
    /// Checks that the given `tile_pos` lies within the extents of the underlying map.
    pub fn checked_remove(&mut self, tile_pos: &TilePos) -> Option<Entity> {
        self.tiles.get_mut(tile_pos.to_index(&self.size))?.take()
    }

    /// Removes all stored `Entity`s, leaving `None` in their place and
    /// returning them in an iterator.
    ///
    /// Example:
    /// ```
    /// # use bevy::prelude::Commands;
    /// # use bevy_ecs_tilemap::prelude::{TilemapSize, TileStorage};
    /// # fn example(mut commands: Commands) {
    /// # let mut storage = TileStorage::empty(TilemapSize { x: 16, y: 16 });
    /// for entity in storage.drain() {
    ///   commands.entity(entity).despawn();
    /// }
    /// # }
    /// ```
    pub fn drain(&mut self) -> impl Iterator<Item = Entity> + use<'_> {
        self.tiles.iter_mut().filter_map(|opt| opt.take())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Entity;

    fn e(id: u32) -> Entity {
        // Helper that makes a deterministic dummy `Entity`
        Entity::from_raw(id)
    }

    fn size_3x3() -> TilemapSize {
        TilemapSize { x: 3, y: 3 }
    }

    #[test]
    fn empty_storage_is_filled_with_none() {
        let storage = TileStorage::empty(size_3x3());
        assert_eq!(storage.size, size_3x3());
        assert!(storage.iter().all(|opt| opt.is_none()));
    }

    #[test]
    fn set_and_get_roundtrip() {
        let mut storage = TileStorage::empty(size_3x3());
        let pos = TilePos { x: 1, y: 2 };
        storage.set(&pos, e(42));
        assert_eq!(storage.get(&pos), Some(e(42)));
    }

    #[test]
    fn checked_get_respects_bounds() {
        let storage = TileStorage::empty(size_3x3());
        // In-bounds → None (nothing stored yet)
        assert_eq!(storage.checked_get(&TilePos { x: 0, y: 0 }), None);
        // Out-of-bounds → None, **not** panic
        assert_eq!(storage.checked_get(&TilePos { x: 99, y: 99 }), None);
    }

    #[test]
    #[should_panic]
    fn get_panics_when_out_of_bounds() {
        let storage = TileStorage::empty(size_3x3());
        let _ = storage.get(&TilePos { x: 50, y: 50 });
    }

    #[test]
    fn remove_returns_entity_and_leaves_none() {
        let mut storage = TileStorage::empty(size_3x3());
        let pos = TilePos { x: 2, y: 1 };
        storage.set(&pos, e(7));
        assert_eq!(storage.remove(&pos), Some(e(7)));
        assert_eq!(storage.get(&pos), None);
    }

    #[test]
    fn drain_yields_every_entity_and_empties_storage() {
        let mut storage = TileStorage::empty(size_3x3());
        storage.set(&TilePos { x: 0, y: 0 }, e(1));
        storage.set(&TilePos { x: 1, y: 1 }, e(2));
        storage.set(&TilePos { x: 2, y: 2 }, e(3));

        let mut drained: Vec<_> = storage.drain().collect();
        drained.sort_by_key(|e| e.index());
        assert_eq!(drained, vec![e(1), e(2), e(3)]);
        assert!(storage.iter().all(|opt| opt.is_none()));
    }

    // ───────────────────────────────
    // MapEntities implementation
    // ───────────────────────────────
    use bevy::ecs::entity::EntityMapper;

    struct AddOneMapper;
    impl EntityMapper for AddOneMapper {
        fn get_mapped(&mut self, entity: Entity) -> Entity {
            // Just bump the raw id for test purposes
            e(entity.index() + 1)
        }

        fn set_mapped(&mut self, _source: Entity, _target: Entity) {
            
        }
    }

    #[test]
    fn map_entities_transforms_every_entity() {
        let mut storage = TileStorage::empty(size_3x3());
        storage.set(&TilePos { x: 0, y: 0 }, e(10));
        storage.set(&TilePos { x: 0, y: 1 }, e(11));

        storage.map_entities(&mut AddOneMapper);

        assert_eq!(storage.get(&TilePos { x: 0, y: 0 }), Some(e(11)));
        assert_eq!(storage.get(&TilePos { x: 0, y: 1 }), Some(e(12)));
    }
}
