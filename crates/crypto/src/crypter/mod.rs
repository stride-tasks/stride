use base64::Engine;
use std::{fmt::Display, io::Read};
use zeroize::Zeroize;

#[cfg(test)]
mod tests;

// https://github.com/rust-lang/rust/issues/60551 :(
pub trait AesMode: Sized {
    const KEY_LEN: usize;
    const IV_LEN: usize;
    const TAG_LEN: usize;
}

#[derive(Debug, Clone, Copy)]
pub struct Aes256Ocb;

impl AesMode for Aes256Ocb {
    const KEY_LEN: usize = 32;
    const IV_LEN: usize = 12;
    const TAG_LEN: usize = 16;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Encryption,
    Decryption,
}

impl std::error::Error for Error {}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "decryption error")
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct Crypter {
    encryption_key: [u8; Aes256Ocb::KEY_LEN],
}

impl Zeroize for Crypter {
    fn zeroize(&mut self) {
        self.encryption_key.zeroize();
    }
}

/// Automatically zero out the contents of the memory when the struct is dropped.
impl Drop for Crypter {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl Crypter {
    #[must_use]
    pub fn new(key: [u8; Aes256Ocb::KEY_LEN]) -> Self {
        Self {
            encryption_key: key,
        }
    }

    /// Generate cryptographic keys from `getrandom` crate.
    ///
    /// # Panics
    ///
    /// If `getrandom` fails.
    #[must_use]
    pub fn generate() -> Self {
        let mut key = [0u8; Aes256Ocb::KEY_LEN];
        getrandom::getrandom(&mut key).expect("could not get random");
        Self {
            encryption_key: key,
        }
    }

    pub fn encrypt(&self, data: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
        let mut iv = [0u8; Aes256Ocb::IV_LEN];
        getrandom::getrandom(&mut iv).expect("cannot get random");

        let mut tag = [0u8; Aes256Ocb::TAG_LEN];
        let ciphertext = openssl::symm::encrypt_aead(
            openssl::symm::Cipher::aes_256_ocb(),
            &self.encryption_key,
            Some(&iv),
            aad,
            data,
            &mut tag,
        )
        .map_err(|_| Error::Encryption)?;

        let mut result = Vec::new();
        result.extend_from_slice(aad);
        result.extend_from_slice(&iv);
        result.extend_from_slice(&tag);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub fn encrypt_with_nonce(&self, nounce: u64, data: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
        let bytes = nounce.to_be_bytes();
        let mut iv = [0u8; Aes256Ocb::IV_LEN];
        iv[..bytes.len()].copy_from_slice(&bytes);
        self.encrypt_with_iv(&iv, data, aad)
    }

    pub fn encrypt_with_iv(
        &self,
        iv: &[u8; Aes256Ocb::IV_LEN],
        data: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>> {
        let mut tag = [0u8; Aes256Ocb::TAG_LEN];
        let ciphertext = openssl::symm::encrypt_aead(
            openssl::symm::Cipher::aes_256_ocb(),
            &self.encryption_key,
            Some(iv),
            aad,
            data,
            &mut tag,
        )
        .map_err(|_| Error::Encryption)?;

        let mut result = Vec::new();
        result.extend_from_slice(aad);
        result.extend_from_slice(iv);
        result.extend_from_slice(&tag);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub fn decrypt_with_nonce<'a>(
        &self,
        nounce: u64,
        data: &'a [u8],
        aad_len: usize,
    ) -> Result<(&'a [u8], Vec<u8>)> {
        let bytes = nounce.to_be_bytes();
        let mut iv = [0u8; Aes256Ocb::IV_LEN];
        iv[..bytes.len()].copy_from_slice(&bytes);
        self.decrypt_with_iv(&iv, data, aad_len)
    }

    pub fn decrypt_with_iv<'a>(
        &self,
        iv: &[u8; Aes256Ocb::IV_LEN],
        data: &'a [u8],
        aad_len: usize,
    ) -> Result<(&'a [u8], Vec<u8>)> {
        let mut tag = [0u8; Aes256Ocb::TAG_LEN];

        if data.len() < aad_len + Aes256Ocb::TAG_LEN {
            return Err(Error::Decryption);
        }

        let aad = &data[..aad_len];

        let mut data = &data[aad_len..];
        data.read_exact(&mut tag).map_err(|_| Error::Decryption)?;

        let plaintext = openssl::symm::decrypt_aead(
            openssl::symm::Cipher::aes_256_ocb(),
            &self.encryption_key,
            Some(iv),
            aad,
            data,
            &tag,
        )
        .map_err(|_| Error::Decryption)?;

        Ok((aad, plaintext))
    }

    pub fn decrypt<'a>(
        &self,
        data: &'a [u8],
        aad_len: usize,
    ) -> Result<(&'a [u8], [u8; Aes256Ocb::IV_LEN], Vec<u8>)> {
        let mut iv = [0u8; Aes256Ocb::IV_LEN];
        let mut tag = [0u8; Aes256Ocb::TAG_LEN];

        if data.len() < aad_len + Aes256Ocb::TAG_LEN {
            return Err(Error::Decryption);
        }

        let aad = &data[..aad_len];

        let mut data = &data[aad_len..];
        data.read_exact(&mut iv).map_err(|_| Error::Decryption)?;
        data.read_exact(&mut tag).map_err(|_| Error::Decryption)?;

        let plaintext = openssl::symm::decrypt_aead(
            openssl::symm::Cipher::aes_256_ocb(),
            &self.encryption_key,
            Some(&iv),
            aad,
            data,
            &tag,
        )
        .map_err(|_| Error::Decryption)?;

        Ok((aad, iv, plaintext))
    }

    #[must_use]
    pub fn to_base64(self) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(self.encryption_key)
    }

    #[must_use]
    pub fn encryption_key(&self) -> &[u8] {
        &self.encryption_key
    }
}
