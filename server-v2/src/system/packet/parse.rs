use crate::ecs::prelude::*;
use crate::protocol::client::{Ack, ScoreDetailed};
use crate::protocol::{ClientPacket as ProtocolClientPacket, ProtocolSerializationExt};
use crate::resource::packet::*;
use crate::resource::socket::MessageEvent;

use airmash_protocol_v5::*;

#[derive(SystemData)]
struct SystemData<'a> {
    ack: Write<'a, OnAck>,
    backup: Write<'a, OnBackup>,
    chat: Write<'a, OnChat>,
    command: Write<'a, OnCommand>,
    horizon: Write<'a, OnHorizon>,
    key: Write<'a, OnKey>,
    local_ping: Write<'a, OnLocalPing>,
    login: Write<'a, OnLogin>,
    pong: Write<'a, OnPong>,
    say: Write<'a, OnSay>,
    score_detailed: Write<'a, OnScoreDetailed>,
    team_chat: Write<'a, OnTeamChat>,
    vote_mute: Write<'a, OnVoteMute>,
    whisper: Write<'a, OnWhisper>,

    unknown: Write<'a, OnUnknown>,
}

macro_rules! single_write {
	{
		match [$evt:ident] $packet:ident {
			$(
				$ty:ident $( ($inner:ident) )? => $channel:expr
			),* $(,)?
		}
	} => {
		match $packet {
			$(
				ProtocolClientPacket::$ty $( ( $inner) )? => single_write!([$evt] $ty $( ( $inner ) )? => $channel),
			)*
		}
	};
	([$evt:expr] $ty:ident => $channel:expr) => {
		single_write!($channel, $evt, $ty)
	};
	([$evt:expr] $ty:ident ($inner:ident) => $channel:expr) => {
		single_write!($channel, $evt, $inner)
	};
	($channel:expr, $evt:expr, $packet:expr) => {
		$channel.single_write(ClientPacket {
			connection: $evt.socket,
			packet: $packet
		})
	}
}

#[event_handler(vis = pub)]
fn handle_message<'a>(evt: &MessageEvent, data: &mut SystemData<'a>) {
    let protocol = ProtocolV5 {};

    let packet: ProtocolClientPacket = match protocol.deserialize(&evt.data) {
        Ok(packet) => packet,
        Err(e) => {
            debug!("Got invalid packet from socket {}: {}", evt.socket, e);

            data.unknown.single_write(ClientPacket {
                connection: evt.socket,
                packet: evt.data.to_vec(),
            });

            return;
        }
    };

    #[rustfmt::skip]
	single_write! {
		match [evt] packet {
			Ack 				=> data.ack,
			Backup(backup) 		=> data.backup,
			Chat(chat)			=> data.chat,
			Command(command)	=> data.command,
			Horizon(horizon)	=> data.horizon,
			Key(key)			=> data.key,
			LocalPing(ping)		=> data.local_ping,
			Login(login)		=> data.login,
			Pong(pong)			=> data.pong,
			Say(say)			=> data.say,
			ScoreDetailed		=> data.score_detailed,
			TeamChat(team_chat)	=> data.team_chat,
			VoteMute(votemute)	=> data.vote_mute,
			Whisper(whisper)	=> data.whisper
		}
	}
}
