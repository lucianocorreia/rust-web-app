mod error;
pub mod pwd;

pub use self::error::{Error, Result};

use hmac::{Hmac, Mac};
use sha2::Sha512;

pub struct EncryptContent {
	pub content: String,
	pub salt: String,
}

pub fn encrypt_into_b64(key: &[u8], enc_content: &EncryptContent) -> Result<String> {
	let EncryptContent { content, salt } = enc_content;

	// -- Create a HMAC-SHA-512 from key
	let mut hmac_sha512 =
		Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

	// -- Add content
	hmac_sha512.update(content.as_bytes());
	hmac_sha512.update(salt.as_bytes());

	// -- Finalize and b64 encode
	let hmac_result = hmac_sha512.finalize();
	let result_bytes = hmac_result.into_bytes();

	let result = base64_url::encode(&result_bytes);

	Ok(result)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
	use rand::RngCore;

	#[test]
	fn test_encrypt_into_b64_ok() -> Result<()> {
		// Arrange
		let mut fx_key = [0u8; 64]; // 512 bits = 64 bytes
		rand::thread_rng().fill_bytes(&mut fx_key);
		let fx_enc_content = EncryptContent {
			content: "hello world".to_string(),
			salt: "some pepper".to_string(),
		};

		// TODO: Need a fix fx_key, and precomputed fx_res
		let fx_res = encrypt_into_b64(&fx_key, &fx_enc_content)?;

		// Act
		let res = encrypt_into_b64(&fx_key, &fx_enc_content)?;

		// Assert
		assert_eq!(fx_res, res);

		Ok(())
	}
}
// endregion: --- Tests
