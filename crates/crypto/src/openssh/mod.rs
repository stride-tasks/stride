#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::struct_field_names)]

use std::{
    io::{BufRead, Read},
    str::FromStr,
};

use base64::Engine;

#[cfg(test)]
mod tests;

// Source: https://dnaeon.github.io/openssh-private-key-binary-format/
//
// ```txt
// ;; AUTH_MAGIC is a hard-coded, null-terminated string,
// ;; set to "openssh-key-v1".
// byte[n] AUTH_MAGIC
//
// ;; ciphername determines the cipher name (if any),
// ;; or is set to "none", when no encryption is used.
// string   ciphername
//
// ;; kdfname determines the KDF function name, which is
// ;; either "bcrypt" or "none"
// string   kdfname
//
// ;; kdfoptions field.
// ;; This one is actually a buffer with size determined by the
// ;; uint32 value, which preceeds it.
// ;; If no encryption was used to protect the private key,
// ;; it's contents will be the [0x00 0x00 0x00 0x00] bytes (empty string).
// ;; You should read the embedded buffer, only if it's size is
// ;; different than 0.
// uint32 (size of buffer)
//     string salt
//     uint32 rounds
//
// ;; Number of keys embedded within the blob.
// ;; This value is always set to 1, at least in the
// ;; current implementation of the private key format.
// uint32 number-of-keys
//
// ;; Public key section.
// ;; This one is a buffer, in which the public key is embedded.
// ;; Size of the buffer is determined by the uint32 value,
// ;; which preceeds it.
// ;; ED25519 public key components.
// uint32 (size of buffer)
//     string keytype ("ssh-ed25519")
//
//     ;; The ED25519 public key is a buffer of size 32.
//     ;; The encoding follows the same rules for any
//     ;; other buffer used by SSH -- the size of the
//     ;; buffer preceeds the actual data.
//     uint32 + byte[32]
//
// ;; Encrypted section
// ;; This one is a again a buffer with size
// ;; specified by the uint32 value, which preceeds it.
// ;; ED25519 private key.
// uint32 (size of buffer)
//     uint32  check-int
//     uint32  check-int  (must match with previous check-int value)
//     string  keytype    ("ssh-ed25519")
//
//     ;; The public key
//     uint32 + byte[32]  (public key)
//
//     ;; Secret buffer. This is a buffer with size 64 bytes.
//     ;; The bytes[0..32] contain the private key and
//     ;; bytes[32..64] contain the public key.
//     ;; Once decoded you can extract the private key by
//     ;; taking the byte[0..32] slice.
//     uint32 + byte[64]  (secret buffer)
//
//     string  comment    (Comment associated with the key)
//     byte[n] padding    (Padding according to the rules above)
// ```
//
// Other sources:
// - https://coolaj86.com/articles/the-openssh-private-key-format
// - https://cvsweb.openbsd.org/src/usr.bin/ssh/PROTOCOL.key?annotate=HEAD
// - https://cvsweb.openbsd.org/src/usr.bin/ssh/PROTOCOL.certkeys?annotate=HEAD
//

const AUTH_MAGIC: &str = "openssh-key-v1\0";

#[derive(Debug)]
pub(crate) enum ParseError {
    InvalidBase64,
    InvalidAuthMagic,
    KeyCountNotOne { count: u32 },
    MismatchCheckint,
    UnknownKeyType { name: Box<str> },
    UnknownCipherName { name: Box<str> },
    UnknownKdfName { name: Box<str> },
    MismatchPublicKey,
    TrailingBytes,
    AbruptEnd,
    Utf8Error(std::str::Utf8Error),
    InvalidSecretBufferLength,
}

impl From<std::str::Utf8Error> for ParseError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBase64 => f.write_str("invalid base64"),
            Self::InvalidAuthMagic => {
                write!(f, "invalid auth magic, expected {AUTH_MAGIC}")
            }
            Self::KeyCountNotOne { count } => {
                write!(f, "invalid key count {count}, expected 1")
            }
            Self::MismatchCheckint => f.write_str("checkint field mismatch"),
            Self::UnknownKeyType { name } => write!(f, "unknown key type: {name}"),
            Self::UnknownCipherName { name } => write!(f, "unknown cipher name: {name}"),
            Self::UnknownKdfName { name } => {
                write!(f, "unknown key derivation function name: {name}")
            }
            Self::MismatchPublicKey => {
                write!(
                    f,
                    "public key does not match public key in the private section"
                )
            }
            Self::TrailingBytes => f.write_str("trailing bytes"),
            Self::AbruptEnd => f.write_str("abrupt end, expected more bytes"),
            Self::Utf8Error(error) => write!(f, "invalid utf-8 error: {error}"),
            Self::InvalidSecretBufferLength => f.write_str("invalid secret buffer length"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum KeyType {
    Ed25519,
}

impl KeyType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ed25519 => "ssh-ed25519",
        }
    }
}

impl FromStr for KeyType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ssh-ed25519" => Ok(KeyType::Ed25519),
            name => Err(ParseError::UnknownKeyType {
                name: name.to_owned().into_boxed_str(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CipherName {
    None,
}

impl CipherName {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }

    fn blocksize(self) -> usize {
        match self {
            Self::None => 8,
        }
    }
}

impl FromStr for CipherName {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(CipherName::None),
            name => Err(ParseError::UnknownCipherName {
                name: name.to_owned().into_boxed_str(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KdfName {
    None,
}

impl KdfName {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl FromStr for KdfName {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(KdfName::None),
            name => Err(ParseError::UnknownKdfName {
                name: name.to_owned().into_boxed_str(),
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PublicSection {
    keytype: KeyType,
    pub0: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
struct PrivateSection {
    checkint: u32,
    keytype: KeyType,
    priv0: Vec<u8>,
}

// TODO: The current implemention is tied to the ED25519
//       and requires some work to make it compatible with RSA.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PrivateKey {
    cipher_name: CipherName,
    kdf_name: KdfName,
    kdf: Vec<u8>,
    public_section: PublicSection,
    private_section: PrivateSection,
    comment: String,
}

impl PrivateKey {
    pub(crate) fn new_ed25519_unencrypted(
        public_key: Vec<u8>,
        private_key: Vec<u8>,
        comment: String,
    ) -> Self {
        let keytype = KeyType::Ed25519;

        let mut checkint_bytes = [0u8; size_of::<u32>()];
        getrandom::fill(&mut checkint_bytes).expect("unable to get random bytes");
        let checkint = u32::from_be_bytes(checkint_bytes);

        Self {
            cipher_name: CipherName::None,
            kdf_name: KdfName::None,
            kdf: vec![],
            public_section: PublicSection {
                keytype,
                pub0: public_key,
            },
            private_section: PrivateSection {
                checkint,
                keytype,
                priv0: private_key,
            },
            comment,
        }
    }
    fn write_bytes(bytes: impl AsRef<[u8]>, result: &mut Vec<u8>) {
        let bytes = bytes.as_ref();
        result.extend((bytes.len() as u32).to_be_bytes());
        result.extend_from_slice(bytes);
    }
    fn public_section(&self, result: &mut Vec<u8>) {
        let length_index = result.len();
        result.extend_from_slice(&u32::MAX.to_be_bytes());

        let start_index = result.len();

        Self::write_bytes(self.public_section.keytype.as_str(), result);
        Self::write_bytes(&self.public_section.pub0, result);

        let end_index = result.len();
        let len = end_index - start_index;

        result[length_index..length_index + 4].copy_from_slice(&(len as u32).to_be_bytes());
    }
    fn private_section(&self, result: &mut Vec<u8>) {
        let length_index = result.len();
        result.extend_from_slice(&u32::MAX.to_be_bytes());

        let start_index = result.len();

        result.extend(self.private_section.checkint.to_be_bytes());
        result.extend(self.private_section.checkint.to_be_bytes());

        Self::write_bytes(self.private_section.keytype.as_str(), result);
        Self::write_bytes(&self.public_section.pub0, result);

        // Secret buffer.
        match self.private_section.keytype {
            KeyType::Ed25519 => {
                // For ED25519:
                //
                // ;; Secret buffer. This is a buffer with size 64 bytes.
                // ;; The bytes[0..32] contain the private key and
                // ;; bytes[32..64] contain the public key.
                // ;; Once decoded you can extract the private key by
                // ;; taking the byte[0..32] slice.
                // uint32 + byte[64]  (secret buffer)
                result.extend(
                    ((self.private_section.priv0.len() + self.public_section.pub0.len()) as u32)
                        .to_be_bytes(),
                );
                result.extend_from_slice(&self.private_section.priv0);
                result.extend_from_slice(&self.public_section.pub0);
            }
        }

        Self::write_bytes(&self.comment, result);

        let end_index = result.len();
        let len = end_index - start_index;

        let blocksize = self.cipher_name.blocksize();
        let pad = blocksize - len % blocksize;
        for i in 0..pad as u8 {
            result.push(i + 1);
        }

        let count = result.len() - start_index;

        result[length_index..length_index + 4].copy_from_slice(&(count as u32).to_be_bytes());
    }
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(AUTH_MAGIC.as_bytes());

        Self::write_bytes(self.cipher_name.as_str(), &mut result);
        Self::write_bytes(self.kdf_name.as_str(), &mut result);
        Self::write_bytes(&self.kdf, &mut result);

        // ;; Number of keys embedded within the blob.
        // ;; This value is always set to 1, at least in the
        // ;; current implementation of the private key format.
        // uint32 number-of-keys
        result.extend(1u32.to_be_bytes());

        self.public_section(&mut result);
        self.private_section(&mut result);

        result
    }

    pub(crate) fn encode_pem(&self) -> String {
        let encoded = self.encode();
        let base64 = base64::engine::general_purpose::STANDARD.encode(encoded);

        let mut result = Vec::new();
        result.extend(b"-----BEGIN OPENSSH PRIVATE KEY-----\n");
        for line in base64.as_bytes().chunks(70) {
            result.extend_from_slice(line);
            result.push(b'\n');
        }
        result.extend(b"-----END OPENSSH PRIVATE KEY-----\n");
        std::str::from_utf8(result.as_slice())
            .expect("base64 encoded string only contains ASCII")
            .to_string()
    }
}

trait ReadExt: Sized {
    fn read(bytes: &mut &[u8]) -> Result<Self, ParseError>;
}
impl ReadExt for u32 {
    fn read(bytes: &mut &[u8]) -> Result<Self, ParseError> {
        let mut buf = [0u8; size_of::<u32>()];
        bytes
            .read_exact(&mut buf)
            .map_err(|_| ParseError::AbruptEnd)?;
        Ok(u32::from_be_bytes(buf))
    }
}

impl ReadExt for Vec<u8> {
    fn read(bytes: &mut &[u8]) -> Result<Self, ParseError> {
        let len = u32::read(bytes)?;
        let (result, new_bytes) = bytes
            .split_at_checked(len as usize)
            .ok_or(ParseError::AbruptEnd)?;
        *bytes = new_bytes;
        Ok(Vec::from(result))
    }
}

impl FromStr for PrivateKey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Should parse header and footer, i.e. -----BEGIN----

        let key = s.split('\n').map(str::trim).collect::<String>();
        let key = base64::engine::general_purpose::STANDARD
            .decode(&key)
            .map_err(|_| ParseError::InvalidBase64)?;

        let mut bytes = key.as_slice();

        let mut auth_magic = Vec::new();
        bytes
            .read_until(b'\0', &mut auth_magic)
            .map_err(|_| ParseError::AbruptEnd)?;

        if auth_magic != AUTH_MAGIC.as_bytes() {
            return Err(ParseError::InvalidAuthMagic);
        }

        let cipher_name = Vec::read(&mut bytes)?;
        let cipher_name = std::str::from_utf8(&cipher_name)?.parse()?;

        let kdf_name = Vec::read(&mut bytes)?;
        let kdf_name = std::str::from_utf8(&kdf_name)?.parse()?;

        let kdf = Vec::read(&mut bytes)?;

        let key_count = u32::read(&mut bytes)?;
        if key_count != 1 {
            return Err(ParseError::KeyCountNotOne { count: key_count });
        }

        let sshpub = Vec::read(&mut bytes)?;
        let mut sshpub_slice = sshpub.as_slice();

        let pub_keytype = Vec::read(&mut sshpub_slice)?;
        let pub_keytype = std::str::from_utf8(&pub_keytype)?.parse()?;
        let pub0 = Vec::read(&mut sshpub_slice)?;

        assert_eq!(sshpub_slice.len(), 0);

        let sshpriv = Vec::read(&mut bytes)?;
        let mut sshpriv_slice = sshpriv.as_slice();

        let dummy_checksum_1 = u32::read(&mut sshpriv_slice)?;
        let dummy_checksum_2 = u32::read(&mut sshpriv_slice)?;

        if dummy_checksum_1 != dummy_checksum_2 {
            return Err(ParseError::MismatchCheckint);
        }

        let priv_keytype = Vec::read(&mut sshpriv_slice)?;
        let priv_keytype = std::str::from_utf8(&priv_keytype)?.parse()?;
        let priv_pub0 = Vec::read(&mut sshpriv_slice)?;
        let mut priv0 = Vec::read(&mut sshpriv_slice)?;

        if pub0 != priv_pub0 {
            return Err(ParseError::MismatchPublicKey);
        }

        match pub_keytype {
            KeyType::Ed25519 => {
                if priv0.len() != 64 {
                    return Err(ParseError::InvalidSecretBufferLength);
                }

                let secret_pub = &priv0[32..64];
                if pub0 != secret_pub {
                    return Err(ParseError::MismatchPublicKey);
                }
                priv0.resize(32, 0);
            }
        }

        let comment = Vec::read(&mut sshpriv_slice)?;
        let comment = std::str::from_utf8(&comment)?.to_owned();

        if !bytes.is_empty() {
            return Err(ParseError::TrailingBytes);
        }

        Ok(PrivateKey {
            cipher_name,
            kdf_name,
            kdf,
            public_section: PublicSection {
                keytype: pub_keytype,
                pub0,
            },
            private_section: PrivateSection {
                checkint: dummy_checksum_1,
                keytype: priv_keytype,
                priv0,
            },
            comment,
        })
    }
}
