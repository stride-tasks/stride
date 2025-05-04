use crate::base64_decode;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptionKey {
    pub key: String,
}

impl EncryptionKey {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn generate() -> Self {
        let key = stride_crypto::crypter::Crypter::generate();
        Self {
            key: key.to_base64(),
        }
    }

    #[must_use]
    /// flutter_rust_bridge:sync
    pub fn validate(key: &str) -> Option<String> {
        let Ok(decoded) = base64_decode(key) else {
            return Some("invalid base64".to_owned());
        };

        if decoded.len() != 32 {
            return Some("encryption key must be ${32 * 8} bits".to_owned());
        }
        None
    }
}
