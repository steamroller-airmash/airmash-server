use super::*;

use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn world_stores_first_item() {
    static CTR: AtomicUsize = AtomicUsize::new(0);

    struct Test(pub usize);
    impl Default for Test {
        fn default() -> Self {
            Self(CTR.fetch_add(1, Ordering::Relaxed))
        }
    }

    let mut world = World::new();

    for _ in 0..10 {
        Read::<Test>::setup(&mut world);
    }

    let test = Read::<Test>::fetch(&world);
    assert_eq!(test.0, 0);
}

#[test]
fn register_storage_twice() {
    #[derive(Copy, Clone, Debug, Default, Component)]
    struct Test;

    let mut world = World::new();

    ReadStorage::<Test>::setup(&mut world);
    ReadStorage::<Test>::setup(&mut world);
}

#[test]
fn units_have_different_typeids() {
    use crate::{Energy, EnergyRegen};
    use std::any::TypeId;

    assert_ne!(TypeId::of::<Energy>(), TypeId::of::<EnergyRegen>());
}
