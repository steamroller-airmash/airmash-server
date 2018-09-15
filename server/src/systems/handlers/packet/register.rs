use dispatch::Builder;

use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		// Add handlers here
		.with::<OnOpenHandler>()
		.with::<OnCloseHandler>()
		.with::<LoginHandler>()
		.with::<KeyHandler>()
		.with::<ChatHandler>()
		.with::<SayHandler>()
		.with::<PongHandler>()
		.with::<ScoreBoardTimerHandler>()
		.with::<PingTimerHandler>()
		.with::<SignalHandler>()
		.with::<WhisperHandler>()
		.with::<ChatEventHandler>()
}
