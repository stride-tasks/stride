use std::fmt::Debug;

use base64::Engine;
use openssl::pkey::PKey;
use pem::Pem;

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
        let rsa = PKey::generate_ed25519().unwrap();

        let public_key = rsa.raw_public_key().unwrap();
        let private_key = rsa.private_key_to_der().unwrap();

        let private_pem = Pem::new("PRIVATE KEY", private_key);
        let private = pem::encode(&private_pem);

        let mut data = Vec::new();
        data.extend((Self::SSH_ED25519.len() as u32).to_be_bytes());
        data.extend(Self::SSH_ED25519.as_bytes());

        data.extend((public_key.len() as u32).to_be_bytes());
        data.extend(public_key);

        // https://coolaj86.com/articles/the-openssh-private-key-format/

        let public = base64::engine::general_purpose::STANDARD.encode(data);

        Self {
            private,
            public: format!("{} {public}", Self::SSH_ED25519),
        }
    }
}
