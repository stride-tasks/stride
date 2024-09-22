use base64::Engine;
use openssl::pkey::PKey;
use std::fmt::Debug;

use crate::openssh::PrivateKey;

pub struct Ed25519 {
    pub public: String,
    pub private: String,
}

impl Debug for Ed25519 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Ed25519")
    }
}

impl Ed25519 {
    pub const SSH_ED25519: &'static str = "ssh-ed25519";

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::missing_panics_doc)]
    pub fn generate() -> Self {
        let key = PKey::generate_ed25519().unwrap();

        let public_key = key.raw_public_key().unwrap();
        let private_key = key.raw_private_key().unwrap();

        assert_eq!(public_key.len(), 32);
        assert_eq!(private_key.len(), 32);

        let mut data = Vec::new();
        data.extend((Self::SSH_ED25519.len() as u32).to_be_bytes());
        data.extend(Self::SSH_ED25519.as_bytes());

        data.extend((public_key.len() as u32).to_be_bytes());
        data.extend(&public_key);

        let public = base64::engine::general_purpose::STANDARD.encode(data);

        let private_key =
            PrivateKey::new_ed25519_unencrypted(public_key, private_key, String::new());

        Self {
            private: private_key.encode_pem(),
            public: format!("{} {public}", Self::SSH_ED25519),
        }
    }
}
