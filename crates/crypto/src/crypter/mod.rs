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
pub struct Aes128Ocb;

impl AesMode for Aes128Ocb {
    const KEY_LEN: usize = 16;
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
pub struct Crypter<const AAD: usize = 0> {
    encryption_key: [u8; Aes128Ocb::KEY_LEN],
}

impl<const AAD: usize> Zeroize for Crypter<AAD> {
    fn zeroize(&mut self) {
        self.encryption_key.zeroize();
    }
}

/// Automatically zero out the contents of the memory when the struct is dropped.
impl<const AAD: usize> Drop for Crypter<AAD> {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl<const AAD: usize> Crypter<AAD> {
    #[must_use]
    pub fn new(key: [u8; Aes128Ocb::KEY_LEN]) -> Self {
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
        let mut key = [0u8; Aes128Ocb::KEY_LEN];
        getrandom::getrandom(&mut key).expect("could not get random");
        Self {
            encryption_key: key,
        }
    }

    pub fn encrypt(&self, nounce: u64, data: &[u8], aad: &[u8; AAD]) -> Result<Vec<u8>> {
        let bytes = nounce.to_be_bytes();
        let mut iv = [0u8; Aes128Ocb::IV_LEN];
        iv[..bytes.len()].copy_from_slice(&bytes);
        self.encrypt_with_iv(&iv, data, aad)
    }

    pub fn encrypt_with_iv(
        &self,
        iv: &[u8; Aes128Ocb::IV_LEN],
        data: &[u8],
        aad: &[u8; AAD],
    ) -> Result<Vec<u8>> {
        let mut tag = [0u8; Aes128Ocb::TAG_LEN];
        let ciphertext = openssl::symm::encrypt_aead(
            openssl::symm::Cipher::aes_128_ocb(),
            &self.encryption_key,
            Some(iv),
            aad,
            data,
            &mut tag,
        )
        .map_err(|_| Error::Encryption)?;

        let mut result = Vec::new();
        result.extend_from_slice(aad);
        // result.extend_from_slice(iv);
        result.extend_from_slice(&tag);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub fn decrypt(&self, nounce: u64, data: &[u8]) -> Result<([u8; AAD], Vec<u8>)> {
        let bytes = nounce.to_be_bytes();
        let mut iv = [0u8; Aes128Ocb::IV_LEN];
        iv[..bytes.len()].copy_from_slice(&bytes);
        self.decrypt_with_iv(&iv, data)
    }

    pub fn decrypt_with_iv(
        &self,
        iv: &[u8; Aes128Ocb::IV_LEN],
        mut data: &[u8],
    ) -> Result<([u8; AAD], Vec<u8>)> {
        let mut aad = [0u8; AAD];
        let mut tag = [0u8; Aes128Ocb::TAG_LEN];

        if data.len() < AAD + Aes128Ocb::TAG_LEN {
            return Err(Error::Decryption);
        }

        data.read_exact(&mut aad).map_err(|_| Error::Decryption)?;
        data.read_exact(&mut tag).map_err(|_| Error::Decryption)?;

        let plaintext = openssl::symm::decrypt_aead(
            openssl::symm::Cipher::aes_128_ocb(),
            &self.encryption_key,
            Some(iv),
            &aad,
            data,
            &tag,
        )
        .map_err(|_| Error::Decryption)?;

        Ok((aad, plaintext))
    }
}