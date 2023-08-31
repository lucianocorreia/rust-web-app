use crate::{Error, Result};
use std::{env, sync::OnceLock};

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env().unwrap_or_else(|ex| {
			panic!("Failed to load config: {ex:?}");
		})
	})
}

#[allow(non_snake_case)]
pub struct Config {
	// -- Db
	pub DB_URL: String,

	// -- web
	pub WEB_FOLDER: String,
}

impl Config {
	fn load_from_env() -> Result<Self> {
		Ok(Self {
			// -- Db
			DB_URL: get_env("SERVICE_DB_URL")?,

			// -- web
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
		})
	}
}

fn get_env(key: &'static str) -> Result<String> {
	env::var(key).map_err(|_| Error::ConfigMissingEnv(key))
}
