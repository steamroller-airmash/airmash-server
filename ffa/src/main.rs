extern crate airmash_server;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rand;
extern crate shred;
extern crate specs;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;
extern crate clap;
extern crate sentry;
extern crate serde_json;

mod components;
mod gamemode;
mod systems;

#[cfg(test)]
mod tests;

use std::env;
use std::fs::File;

use gamemode::EmptyGameMode;

use airmash_server::*;

/// NOTE: Also initializes env_logger
fn init_sentry() -> Option<sentry::internals::ClientInitGuard> {
    if let Ok(dsn) = env::var("SENTRY_DSN") {
        let guard = sentry::init(&*dsn);

        sentry::integrations::env_logger::init(None, Default::default());
        sentry::integrations::panic::register_panic_handler();

        Some(guard)
    } else {
        env_logger::init();

        None
    }
}

fn set_default_var(name: &str, value: &str)
{
    if None == env::var_os(name) {
        env::set_var(name, value);
    }
}

fn main() {
    let matches = clap::App::new("airmash-server-ffa")
        .version(env!("CARGO_PKG_VERSION"))
        .author("STEAMROLLER")
        .about("Airmash FFA server")
        .args_from_usage("-c, --config=[FILE] 'Provides an alternate config file'")
        .get_matches();

    set_default_var("RUST_BACKTRACE", "1");
    set_default_var("RUST_LOG", "info");
    set_default_var("RAYON_NUM_THREADS", "1");
	set_default_var("FFA_LISTEN_ADDR", "0.0.0.0");
	set_default_var("FFA_LISTEN_PORT", "3501");

	let bind_addr = format!("{}:{}",
		env::var("FFA_LISTEN_ADDR").unwrap(),
		env::var("FFA_LISTEN_PORT").unwrap());

    let _guard = init_sentry();

    let mut config = AirmashServerConfig::new(bind_addr, EmptyGameMode).with_engine();

    config.builder = systems::register(config.builder);

    if let Some(path) = matches.value_of("config") {
        let file = match File::open(path) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Unable to open config file. Error was {}", e);
                return;
            }
        };

        let serverconfig: Config = serde_json::from_reader(file).unwrap_or_else(|e| {
            error!("Unable to parse config file! Using default config.");
            error!("Config file error was: {}", e);
            Default::default()
        });

        config.world.add_resource(serverconfig);
    }

    AirmashServer::new(config).run().unwrap();
}
