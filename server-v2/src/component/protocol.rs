use crate::ecs::{Component, DenseVecStorage, VecStorage};
use crate::protocol::*;

macro_rules! impl_component {
	[$ty:ty : $storage:ident] => {
		impl Component for $ty {
			type Storage = $storage<Self>;
		}
	};
	[$ty:ty] => {
		impl_component![$ty : DenseVecStorage];
	};
	{
		$(
			$ty:ty $( => $storage:ident)?;
		)*
	} => {
		$(
			impl_component![$ty $( : $storage )?];
		)*
	}
}

impl<V, const L: isize, const T: isize, const H: isize, const E: isize, const R: isize> Component
    for AirmashUnits<V, L, T, H, E, R>
{
    type Storage = VecStorage<Self>;
}

impl<T> Component for Vector2<T>
where
	T: Component
{
	type Storage = VecStorage<Self>;
}

impl_component! {
    Upgrades => DenseVecStorage;
    PlayerStatus => DenseVecStorage;
    FlagCode => DenseVecStorage;
    Level => DenseVecStorage;
    Score => DenseVecStorage;
    MobType => DenseVecStorage;
    PlaneType => DenseVecStorage;
    Team => DenseVecStorage;
}
