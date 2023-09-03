use super::{Error, Result};
use crate::config;
use crate::crypt::{encrypt_into_b64, EncryptContent};

/// Encrypt the password with the default scheme
pub fn encrypt_pwd(content: &EncryptContent) -> Result<String> {
	let key = &config().PWD_KEY;

	let encrypted = encrypt_into_b64(key, content)?;

	Ok(format!("#01#{encrypted}"))
}

/// Validate if an EncryptedContent matches
pub fn validate_pwd(content: &EncryptContent, pwd_ref: &str) -> Result<()> {
	let pwd = encrypt_pwd(content)?;

	if pwd == pwd_ref {
		Ok(())
	} else {
		Err(Error::PwdNotMatching)
	}
}
