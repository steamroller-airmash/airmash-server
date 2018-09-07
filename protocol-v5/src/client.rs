use error::*;
use protocol_common::*;
use serde::*;

impl_serde! {
	struct client::Login {
		protocol: u8,
		name: text,
		session: text,
		horizon_x: u16,
		horizon_y: u16,
		flag: text,
	}

	struct client::Backup {
		token: text,
	}

	struct client::Horizon {
		horizon_x: u16,
		horizon_y: u16,
	}

	struct client::Pong {
		num: u32,
	}

	struct client::Key {
		seq: u32,
		key: KeyCode,
		state: bool
	}

	struct client::Command {
		com: text,
		data: text
	}

	struct client::Chat {
		text: text
	}

	struct client::Whisper {
		id: Player,
		text: text
	}

	struct client::Say {
		text: text
	}

	struct client::TeamChat {
		text: text
	}

	struct client::VoteMute {
		id: Player
	}

	struct client::LocalPing {
		auth: u32
	}
}

mod consts {
	pub const LOGIN: u8 = 0;
	pub const BACKUP: u8 = 1;
	pub const HORIZON: u8 = 2;
	pub const ACK: u8 = 5;
	pub const PONG: u8 = 6;
	pub const KEY: u8 = 10;
	pub const COMMAND: u8 = 11;
	pub const SCORE_DETAILED: u8 = 12;
	pub const CHAT: u8 = 20;
	pub const WHISPER: u8 = 21;
	pub const SAY: u8 = 22;
	pub const TEAMCHAT: u8 = 23;
	pub const VOTEMUTE: u8 = 25;
	pub const LOCALPING: u8 = 255;
}

impl Serialize for ClientPacket {
	fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
		use self::consts::*;
		use self::ClientPacket::*;

		// TODO: Implement trace info here

		match self {
			Login(x) => (LOGIN, x).serialize(ser),
			Backup(x) => (BACKUP, x).serialize(ser),
			Horizon(x) => (HORIZON, x).serialize(ser),
			Ack => ACK.serialize(ser),
			Pong(x) => (PONG, x).serialize(ser),
			Key(x) => (KEY, x).serialize(ser),
			Command(x) => (COMMAND, x).serialize(ser),
			ScoreDetailed => SCORE_DETAILED.serialize(ser),
			Chat(x) => (CHAT, x).serialize(ser),
			TeamChat(x) => (TEAMCHAT, x).serialize(ser),
			Whisper(x) => (WHISPER, x).serialize(ser),
			Say(x) => (SAY, x).serialize(ser),
			VoteMute(x) => (VOTEMUTE, x).serialize(ser),
			LocalPing(x) => (LOCALPING, x).serialize(ser),
		}
	}
}

macro_rules! match_case {
	($ty:ident, $de:ident) => {
		$ty(Deserialize::deserialize($de).map_err(|e| {
			e.chain(FieldSpec {
				field: FieldName::Name(stringify!($ty)),
				ty: "ClientPacket".into(),
				})
		})?).into()
	};
}

impl Deserialize for ClientPacket {
	fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
		use self::consts::*;
		use self::ClientPacket::*;

		let val: u8 = de.deserialize_u8().map_err(|e| {
			e.chain(FieldSpec {
				field: FieldName::Name("<variant-number>"),
				ty: "ClientPacket".into(),
			})
		})?;

		Ok(match val {
			LOGIN => match_case!(Login, de),
			BACKUP => match_case!(Backup, de),
			HORIZON => match_case!(Horizon, de),
			ACK => Ack,
			PONG => match_case!(Pong, de),
			KEY => match_case!(Key, de),
			COMMAND => match_case!(Command, de),
			SCORE_DETAILED => ScoreDetailed,
			CHAT => match_case!(Chat, de),
			TEAMCHAT => match_case!(TeamChat, de),
			WHISPER => match_case!(Whisper, de),
			SAY => match_case!(Say, de),
			VOTEMUTE => match_case!(VoteMute, de),
			LOCALPING => match_case!(LocalPing, de),
			x => {
				return Err(DeserializeError {
					ty: DeserializeErrorType::InvalidEnumValue(x as usize),
					trace: vec![FieldSpec {
						field: FieldName::Name("<variant-number>"),
						ty: "ClientPacket".into(),
					}],
				})
			}
		})
	}
}
