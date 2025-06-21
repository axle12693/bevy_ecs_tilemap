use bevy::{ ecs::world::CommandQueue, prelude::* };
use bevy_ecs_tilemap::prelude::{ TilemapSize, TilePos, TileStorage };

#[test]
fn drain_can_be_used_to_despawn_entities() {
    let mut app = App::new();
    // Spawn three entities and store their IDs
    let e1 = app.world_mut().spawn_empty().id();
    let e2 = app.world_mut().spawn_empty().id();
    let e3 = app.world_mut().spawn_empty().id();

    // Put them in a storage component that lives on its own entity
    let mut storage = TileStorage::empty(TilemapSize { x: 2, y: 2 });
    storage.set(&(TilePos { x: 0, y: 0 }), e1);
    storage.set(&(TilePos { x: 1, y: 0 }), e2);
    storage.set(&(TilePos { x: 0, y: 1 }), e3);

    let storage_entity = app.world_mut().spawn(storage).id();

    // Use Commands-style despawning exactly as the docs example shows
    let mut queue = CommandQueue::default();
    {
        let mut storage = app.world_mut().entity_mut(storage_entity).take::<TileStorage>().unwrap();
        let mut commands = Commands::new(&mut queue, &app.world());
        for entity in storage.drain() {
            commands.entity(entity).despawn();
        }
        // Put the (now empty) storage back on the entity
        commands.entity(storage_entity).insert(storage);
    }
    queue.apply(&mut app.world_mut());

    // Verify that the entities are really gone
    assert!(!app.world().entities().contains(e1));
    assert!(!app.world().entities().contains(e2));
    assert!(!app.world().entities().contains(e3));
}
