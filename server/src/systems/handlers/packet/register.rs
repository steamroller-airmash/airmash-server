use dispatch::Builder;

use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		// Add handlers here
		.with::<OnOpenHandler>()
		.with_handler::<OnCloseHandler>()
		.with::<LoginHandler>()
		.with::<KeyHandler>()
		.with::<ChatHandler>()
		.with_handler::<SayHandler>()
		.with::<PongHandler>()
		.with_handler::<ScoreBoardTimerHandler>()
		.with::<PingTimerHandler>()
		.with::<SignalHandler>()
		.with::<WhisperHandler>()
		.with::<ChatEventHandler>()
		.with_handler::<TeamChatHandler>()
}
