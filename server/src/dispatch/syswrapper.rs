use dispatch::sysinfo::*;
use specs::*;

use metrics::MetricsHandler;
use std::time::Instant;

pub struct SystemWrapper<T>(pub T);

impl<'a, T: 'a> System<'a> for SystemWrapper<T>
where
	T: System<'a> + SystemInfo,
	T::SystemData: SystemData<'a>,
{
	type SystemData = (ReadExpect<'a, MetricsHandler>, T::SystemData);

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);
		self.0.setup(res);
	}

	fn run(&mut self, (metrics, data): Self::SystemData) {
		let start = Instant::now();
		self.0.run(data);
		metrics
			.time_duration(T::name(), Instant::now() - start)
			.unwrap();
	}
}
