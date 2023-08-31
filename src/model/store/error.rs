use std::fmt::{Display, Formatter};

use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	FailToCreatePool(String),
}

impl Display for Error {
	fn fmt(
		&self,
		fmt: &mut Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
