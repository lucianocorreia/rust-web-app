use crate::{Error, Result};
use std::{env, str::FromStr, sync::OnceLock};

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
	// -- Crypt
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,

	// -- Db
	pub DB_URL: String,

	// -- Web
	pub WEB_FOLDER: String,
}

impl Config {
	fn load_from_env() -> Result<Self> {
		Ok(Self {
			// -- Crypt
			PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,

			TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
			TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,

			// -- Db
			DB_URL: get_env("SERVICE_DB_URL")?,

			// -- web
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
		})
	}
}

fn get_env_parse<T: FromStr>(arg: &'static str) -> Result<T> {
	let val = get_env(arg)?;
	val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(arg))
}

fn get_env_b64u_as_u8s(arg: &'static str) -> Result<Vec<u8>> {
	base64_url::decode(&get_env(arg)?).map_err(|_| Error::ConfigWrongFormat(arg))
}

fn get_env(key: &'static str) -> Result<String> {
	env::var(key).map_err(|_| Error::ConfigMissingEnv(key))
}
