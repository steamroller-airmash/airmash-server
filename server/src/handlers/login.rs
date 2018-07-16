use specs::*;

use component::channel::*;
use component::event::TimerEvent;
use consts::timer::*;
use types::*;

use std::env;
use std::io::Read as IoRead;
use std::sync::mpsc::*;
use std::sync::Arc;
use std::time::Instant;

use hyper::{Client, Url};

pub struct LoginHandler {
	reader: Option<OnLoginReader>,
	channel: Option<Sender<TimerEvent>>,
	upstream: Option<String>,
	client: Arc<Client>,
}

impl LoginHandler {
	pub fn new() -> Self {
		Self {
			reader: None,
			channel: None,
			upstream: env::var("IP_FILTER").ok(),
			client: Arc::new(Client::new()),
		}
	}
}

impl<'a> System<'a> for LoginHandler {
	type SystemData = (Read<'a, OnLogin>, Read<'a, Connections>);

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnLogin>().register_reader());
		self.channel = Some(res.fetch_mut::<FutureDispatcher>().get_channel());
	}

	fn run(&mut self, (channel, conns): Self::SystemData) {
		if let Some(ref mut reader) = self.reader {
			for evt in channel.read(reader).cloned() {
				let conninfo = conns.0[&evt.0].info.clone();
				let channel = self.channel.as_ref().unwrap().clone();

				let connid = evt.0;

				let mut event = TimerEvent {
					ty: *LOGIN_PASSED,
					instant: Instant::now(),
					data: Some(Box::new(evt)),
				};

				if cfg!(not(features = "block-bots")) {
					channel.send(event).unwrap();
					continue;
				}

				let upstream = self.upstream.clone();
				let mut is_bot = conninfo.origin.is_none();

				if let Some(upstream) = upstream {
					let url = format!("http://{}/{}", upstream, conninfo.addr);
					let url = Url::parse(&url).unwrap();
					let client = Arc::clone(&self.client);

					let is_bot = is_bot || match client.get(url).send() {
						Ok(mut v) => {
							let mut s = "".to_owned();
							v.read_to_string(&mut s).ok();

							match s.parse() {
								Ok(v) => v,
								Err(_) => false,
							}
						}
						Err(_) => false,
					};

					if is_bot {
						event.ty = *LOGIN_FAILED;
					}

					channel.send(event).unwrap();
				} else {
					if is_bot {
						event.ty = *LOGIN_FAILED;
					}

					channel.send(event).unwrap();
				}

				info!(
					"{:?} with addr {:?} and origin {:?} is a bot? {:?}",
					connid, conninfo.addr, conninfo.origin, is_bot
				);
			}
		}
	}
}

use dispatch::SystemInfo;
use handlers::OnCloseHandler;

impl SystemInfo for LoginHandler {
	type Dependencies = OnCloseHandler;

	fn new() -> Self {
		Self::new()
	}

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}
}
