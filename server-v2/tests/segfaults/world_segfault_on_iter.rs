use server_v2::ecs::*;

use std::marker::PhantomData;

#[test]
fn world_iter_storages() {
    #[derive(Copy, Clone, Debug, Default, Component)]
    struct Test<T> {
        _marker: PhantomData<T>,
    }

    let mut world = World::new();

    ReadStorage::<Test<u32>>::setup(&mut world);
    ReadStorage::<Test<i32>>::setup(&mut world);

    let ent = world
        .create_entity()
        .with(Test::<u32>::default())
        .with(Test::<i32>::default())
        .build();

    let entities: Entities = world.system_data();
    entities.delete(ent).expect("No such entity");
    drop(entities);

    world.maintain();
}
