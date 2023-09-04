pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// -- Time
	DateFailParse(String),

	// -- Base64
	FailToB64uDecode,
}

// region:    --- Error impl

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(f, "{self:?}")
	}
}

impl ::std::error::Error for Error {}

// endregion: --- Error impl