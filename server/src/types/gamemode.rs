//! Note: This entire module is an exercise
//! in bashing the trait system over the
//! head until it lets us do what we want
//! safely.

use shred::{ResourceId, SystemData};
use specs::*;

use types::*;

use std::any::Any;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use protocol::GameType;

pub trait GameMode: Any + Sync + Send {
	fn assign_team(&mut self, player: Entity) -> Team;
	fn spawn_pos(&mut self, player: Entity, team: Team) -> Position;
	fn assign_plane(&mut self, _player: Entity, _team: Team) -> Plane {
		Plane::Predator
	}

	fn gametype(&self) -> GameType;
	fn room(&self) -> String;
}

pub trait GameModeWrapper: Send + Sync {
	fn as_gamemode_ref<'a>(&'a self) -> &'a GameMode;
	fn as_gamemode_mut<'a>(&'a mut self) -> &'a mut GameMode;
	fn as_any_ref<'a>(&'a self) -> &'a Any;
	fn as_any_mut<'a>(&'a mut self) -> &'a mut Any;
}

pub struct GameModeWrapperImpl<T: GameMode + Sync + Send + 'static> {
	pub val: T,
}

impl<T> GameModeWrapper for GameModeWrapperImpl<T>
where
	T: GameMode + Sync + Send + 'static,
{
	fn as_gamemode_ref<'a>(&'a self) -> &'a GameMode {
		&self.val
	}
	fn as_gamemode_mut<'a>(&'a mut self) -> &'a mut GameMode {
		&mut self.val
	}

	fn as_any_ref<'a>(&'a self) -> &'a Any {
		&self.val
	}
	fn as_any_mut<'a>(&'a mut self) -> &'a mut Any {
		&mut self.val
	}
}

pub struct GameModeInternal(pub Box<GameModeWrapper>);

pub struct GameModeWriter<'a, T: ?Sized> {
	inner: WriteExpect<'a, GameModeInternal>,
	marker: PhantomData<T>,
}

impl<'a> GameModeWriter<'a, GameMode> {
	pub fn get(&self) -> &GameMode {
		self.inner.0.as_gamemode_ref()
	}
	pub fn get_mut(&mut self) -> &mut GameMode {
		self.inner.0.as_gamemode_mut()
	}
}

impl<'a, T> SystemData<'a> for GameModeWriter<'a, T>
where
	T: GameMode + Sync + Send + ?Sized,
{
	fn setup(res: &mut Resources) {
		WriteExpect::<'a, GameModeInternal>::setup(res);
	}

	fn fetch(res: &'a Resources) -> Self {
		Self {
			inner: WriteExpect::<'a, GameModeInternal>::fetch(res),
			marker: PhantomData,
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
where
	T: GameMode,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match self.inner.0.as_any_ref().downcast_ref() {
			Some(x) => x,
			None => {
				panic!("Game mode was not expected type when using GameModeWriter");
			}
		}
	}
}

impl<'a, T> DerefMut for GameModeWriter<'a, T>
where
	T: GameMode,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self.inner.0.as_any_mut().downcast_mut() {
			Some(x) => x,
			None => {
				panic!("Game mode was not expected type when using GameModeWriter");
			}
		}
	}
}
