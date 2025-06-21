use bevy::ecs::entity::{EntityMapper, MapEntities};
use bevy::prelude::Entity;
use bevy_ecs_tilemap::map::TilemapId;

/// Tiny stub that maps a single entity to a different one.
struct DummyMapper {
    from: Entity,
    to: Entity,
}

impl EntityMapper for DummyMapper {
    fn get_mapped(&mut self, e: Entity) -> Entity {
        if e == self.from { self.to } else { e }
    }

    fn set_mapped(&mut self, _source: Entity, _target: Entity) {}
}

#[test]
fn tilemap_id_is_remapped() {
    let old = Entity::from_raw(1);
    let new = Entity::from_raw(2);

    let mut id = TilemapId(old);
    let mut mapper = DummyMapper { from: old, to: new };

    id.map_entities(&mut mapper);
    assert_eq!(id.0, new);
}
