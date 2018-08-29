use specs::System;
use SystemInfo;

pub trait SystemParent {
	type Inner: for<'c> System<'c> + Send + SystemInfo;
}

impl<T> SystemParent for T
where
	T: for<'c> System<'c> + Send + SystemInfo,
{
	type Inner = T;
}
