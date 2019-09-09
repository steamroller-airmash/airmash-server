//! Systems that are required for important engine
//! features to work.

mod packet_handler;
pub(crate) mod task_timer;
mod timer_handler;

pub use self::packet_handler::PacketHandler;
pub use self::task_timer::TaskTimerSystem as TaskTimer;
pub use self::timer_handler::TimerHandler;

use crate::Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		// These systems are registered within server config
		// .with::<TimerHandler>()
		// .with::<PacketHandler>()
		.with::<TaskTimer>()
}
