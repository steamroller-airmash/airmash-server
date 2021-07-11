//! Airmash CTF server.

use airmash::AirmashGame;

#[macro_use]
extern crate log;
#[macro_use]
extern crate airmash;

pub mod component;
pub mod config;
pub mod event;
pub mod resource;
pub mod shuffle;
mod systems;

fn setup_flag_entities(game: &mut AirmashGame) {
  use crate::component::*;
  use crate::config::{BLUE_TEAM, RED_TEAM};

  use airmash::component::*;
  use std::time::Instant;

  game.world.spawn((
    Position(config::flag_home_pos(RED_TEAM)),
    Team(RED_TEAM),
    FlagCarrier(None),
    LastDrop {
      player: None,
      time: Instant::now(),
    },
    LastReturnTime(Instant::now()),
    IsFlag,
  ));

  game.world.spawn((
    Position(config::flag_home_pos(BLUE_TEAM)),
    Team(BLUE_TEAM),
    FlagCarrier(None),
    LastDrop {
      player: None,
      time: Instant::now(),
    },
    LastReturnTime(Instant::now()),
    IsFlag,
  ));
}

pub fn setup_ctf_server(game: &mut AirmashGame) {
  use airmash::resource::GameType;

  game.resources.insert(GameType::CTF);

  setup_flag_entities(game);
  crate::resource::register_all(game);
  airmash::system::ctf::register_all(game);
}
