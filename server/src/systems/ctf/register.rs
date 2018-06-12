
use systems::ctf::*;
use component::ctf::{IsFlag, FlagCarrier, LastDrop};

use specs::*;
use types::{
	Team,
	Position
};

use std::time::Instant;

pub fn register<'a, 'b>(world: &mut World, disp: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
	world.register::<Team>();
	world.register::<Position>();
	world.register::<IsFlag>();
	world.register::<FlagCarrier>();
	world.register::<LastDrop>();

	let lastdrop = LastDrop {
		player: None,
		time: Instant::now()
	};

	world.create_entity()
		.with(Team(1))
		.with(config::FLAG_POS[&Team(1)])
		.with(IsFlag{})
		.with(FlagCarrier(None))
		.with(lastdrop)
		.build();
	world.create_entity()
		.with(Team(2))
		.with(config::FLAG_POS[&Team(2)])
		.with(IsFlag{})
		.with(FlagCarrier(None))
		.with(lastdrop)
		.build();

	disp
		.with(loginupdate::LoginUpdateSystem::new(),     "ctf_loginupdate", &[])
		.with(pickupflag::PickupFlagSystem{},            "ctf_pickupflag",  &["position_update", "ctf_loginupdate"])
		.with(sendmessage::SendFlagMessageSystem::new(), "ctf_sendmessage", &["ctf_pickupflag"])
		.with(leaveupdate::LeaveUpdateSystem::new(),     "ctf_leaveupdate", &["position_update"])
		.with(drop::DropSystem::new(),                   "ctf_drop",        &["ctf_pickupflag"])
		.with(return_flag::ReturnFlagSystem{},           "ctf_return_flag", &["ctf_pickupflag"])
		.with(pos_update::PosUpdateSystem{},             "ctf_pos_update",  &["ctf_pickupflag"])
		.with(flag_message::PickupMessageSystem::new(),  "ctf_flag_message", &["ctf_return_flag", "ctf_pickupflag"])
}
