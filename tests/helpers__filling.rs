use bevy::ecs::{ system::Commands, world::{ CommandQueue, World } };
use bevy_ecs_tilemap::{
    map::{ TilemapId, TilemapSize },
    prelude::fill_tilemap,
    tiles::{ TilePos, TileStorage, TileTextureIndex },
};

fn spawn_tilemap(world: &mut World) -> (TilemapId, TileStorage) {
    let size = TilemapSize { x: 4, y: 3 };
    let id = TilemapId(world.spawn_empty().id());
    (id, TileStorage::empty(size))
}

#[test]
fn fill_tilemap_fills_every_cell() {
    let mut world = World::default();
    let mut queue = CommandQueue::default();
    let (tilemap_id, mut storage) = spawn_tilemap(&mut world);
    let mut commands = Commands::new(&mut queue, &mut world);

    let size = storage.size;

    fill_tilemap(TileTextureIndex(7), size, tilemap_id, &mut commands, &mut storage);
    queue.apply(&mut world);

    // every position should have an entity and the world should own it
    let mut filled = 0;
    for x in 0..size.x {
        for y in 0..size.y {
            let pos = TilePos { x, y };
            let entity = storage.get(&pos).expect("position not filled");
            assert!(!world.get_entity(entity).is_err());

            filled += 1;
        }
    }
    assert_eq!(filled, (size.x * size.y) as usize);
}
