use std::net::ToSocketAddrs;
use std::sync::mpsc::{channel, Sender};

use dispatch::Builder;
use specs::{Builder as SpecsBuilder, World};

use component::event::TimerEvent;
use types::event::ConnectionEvent;
use types::GameMode;

/// Configuration options for an Airmash server.
pub struct AirmashServerConfig<T>
where
	T: ToSocketAddrs + Send + 'static,
{
	/// The address(es) that the server will be
	/// listening on once it's running.
	pub addr: T,
	/// Specs world, this contains all components
	/// and all resources available to systems
	/// within the server.
	pub world: World,
	/// System builder, register all your systems
	/// here.
	pub builder: Builder<'static, 'static>,
	/// The maximum number of websocket connections
	/// that the server will accept before dropping
	/// further ones. This is a hard limit to the
	/// number of players that can be on a server.
	///
	/// The default is 256.
	pub max_connections: usize,

	pub(super) event: Sender<ConnectionEvent>,
	pub(super) timer: Sender<TimerEvent>,
}

impl<T> AirmashServerConfig<T>
where
	T: ToSocketAddrs + Send + 'static,
{
	/// Unless you know exactly what you're doing, you
	/// probably want [`new`][0].
	///
	/// Creates a config without adding a game mode to it.
	/// Note that there's a few engine systems that assume
	/// a game mode is present, so unless you're overhauling
	/// the entire engine it is necessary to add a game mode
	/// before starting the server.
	///
	/// [0]: #fn.new
	pub fn new_no_gamemode(addr: T) -> Self {
		use systems::{PacketHandler, TimerHandler};
		use types::{Connections, FutureDispatcher};

		let (event_send, event_recv) = channel();
		let (timer_send, timer_recv) = channel();

		// Nothing in the engine will work without these 3 systems,
		// so they need to be registered now.
		let builder = Builder::new()
			.with_args::<PacketHandler, _>(event_recv)
			.with_args::<TimerHandler, _>(timer_recv);

		let mut world = World::new();

		// These two systems take in some out-of-band channels
		// (thus they must be used with ReadExpect). We register
		// them here to avoid having an awkward system where it
		// is necessary to pass Option<Sender<_>> all the way
		// through the config struct.
		world.add_resource(Connections::new());
		world.add_resource(FutureDispatcher::new(timer_send.clone()));

		Self {
			addr,
			world,
			builder,
			max_connections: 256,

			event: event_send,
			timer: timer_send,
		}
		.with_filler_entities()
	}

	/// Creates a new server config with an address to
	/// listen on and a game mode.
	pub fn new<G>(addr: T, gamemode: G) -> Self
	where
		G: GameMode + 'static,
	{
		let me = Self::new_no_gamemode(addr);

		// Technically it's not necessary to add the gamemode within
		// the constructor. However, it's likely that people using
		// the engine for the first time will forget to add it
		// (and it is almost always required) so I've made the
		// default method that people will reach for require it.
		me.with_gamemode(gamemode)
	}

	/// Register all engine systems.
	///
	/// This is everything within the [`systems`][0]
	/// namespace. If more customization is
	/// needed, then you'll have to call the
	/// register methods individually.
	///
	/// # Notes
	/// Normally it wouldn't be necessary
	/// to follow any order when registering systems.
	/// However, due to how some channels are registered
	/// here, it is a good idea to call this before
	/// registering any of your own systems.
	///
	/// [0]: ::systems
	pub fn with_engine(self) -> Self {
		use systems;

		Self {
			builder: systems::register(self.builder),
			..self
		}
	}

	/// Replace an existing game mode within the world.
	///
	/// # Note
	/// This can also be used to add a game mode to a world
	/// if the world was constructed using [`new_no_gamemode`][0].
	///
	/// [0]: #method.new_no_gamemode
	pub fn with_gamemode<G>(mut self, gamemode: G) -> Self
	where
		G: GameMode + 'static,
	{
		use types::gamemode::*;

		self.world
			.add_resource(GameModeInternal(Box::new(GameModeWrapperImpl {
				val: gamemode,
			})));

		self
	}

	/// Add an alpha banner indicating that the server is
	/// under development.
	///
	/// # Note
	/// Sooner or later this will be replaced with something
	/// to add a generic banner.
	pub fn with_alpha_warning(self) -> Self {
		use systems::notify::*;

		Self {
			builder: self.builder.with_handler::<NotifyAlpha>(),
			..self
		}
	}

	/// Add some dummy entities so that there are no players
	/// with id 0, 1, or 2. This means that game modes don't
	/// have to consider some non-obvious edge cases.
	///
	/// The edge cases are as follows:
	/// - Having players with ids 1 or 2 in FFA causes errors
	///   in the client (at least in StarMash) and also leads
	///   to players having blue and red names in FFA.
	/// - Having players with id 0 breaks collisions. The terrain
	///   collision system assumes that there are no players
	///   with id 0, so these players will be able to fly
	///   through walls.
	/// - Having players with id 0 makes all other players
	///   using StarMash imitate the player with id 0.
	///
	/// At the moment this method is private since there isn't
	/// really a case where it's *absolutely* necessary to be
	/// able to create entities with these ids. It may become
	/// public in the future if it can be designed in right.
	fn with_filler_entities(mut self) -> Self {
		while self.world.create_entity().build().id() < 3 {}

		self
	}
}
