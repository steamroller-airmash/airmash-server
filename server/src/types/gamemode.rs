
use specs::*;
use shred::{SystemData, ResourceId};

use types::*;

use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

pub trait GameMode: Sync + Send + Any {
	fn assign_team(&mut self, player: Entity) -> Team;
	fn respawn_pos(&mut self, player: Entity, team: Team) -> Position;
}

pub struct GameModeInternal(Box<Any + Send + Sync>);

pub struct GameModeWriter<'a, T>
where T: GameMode
{
	inner: WriteExpect<'a, GameModeInternal>,
	marker: PhantomData<T>
}

impl<'a, T> SystemData<'a> for GameModeWriter<'a, T> 
where
	T: GameMode,
	WriteExpect<'a, GameModeInternal>: SystemData<'a>,
	GameModeInternal: Any + Send + Sync
{
	fn setup(res: &mut Resources) {
		WriteExpect::<'a, GameModeInternal>::setup(res);
	}

	fn fetch(res: &'a Resources) -> Self {
		Self {
			inner: WriteExpect::<'a, GameModeInternal>::fetch(res),
			marker: PhantomData
		}
	}

	fn reads() -> Vec<ResourceId> {
		WriteExpect::<'a, GameModeInternal>::reads()
	}

	fn writes() -> Vec<ResourceId> {
		WriteExpect::<'a, GameModeInternal>::writes()
	}
}

impl<'a, T> Deref for GameModeWriter<'a, T>
where T: GameMode
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match self.inner.0.downcast_ref() {
			Some(x) => x,
			None => {
				panic!("Game mode was not expected type when using GameModeWriter");
			}
		}
	}
}

impl<'a, T> DerefMut for GameModeWriter<'a, T>
where T: GameMode
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self.inner.0.downcast_mut() {
			Some(x) => x,
			None => {
				panic!("Game mode was not expected type when using GameModeWriter");
			}
		}
	}
}
