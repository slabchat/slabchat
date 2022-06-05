use crate::error::{Error, Result};
use config::Config;
use lazy_static::lazy_static;
use serde::Deserialize;
use tracing::instrument;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
	pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
	pub port: u16,
	pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
	pub uri: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwt {
	pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
	pub server: Server,
	pub database: Database,
	pub log: Log,
	pub jwt: Jwt,
}

impl Settings {
	#[instrument]
	pub fn new() -> Result<Self> {
		let config = Config::builder()
			.add_source(config::File::with_name("config/default"))
			.add_source(config::Environment::with_prefix("SLAB"))
			.build()
			.map_err(|_| Error::LoadConfig)?;

		let s: Settings = config.try_deserialize().unwrap();

		Ok(s)
	}
}

lazy_static! {
	pub static ref CONFIG: Settings =
		Settings::new().expect("config could not be loaded");
}
