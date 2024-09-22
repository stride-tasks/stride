use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use base64::Engine;
use stride_crypto::crypter::Crypter;
use uuid::Uuid;

use crate::{
    api::{error::KeyStoreError, repository::generate_iv},
    task::{Task, TaskStatus},
    ErrorKind, RustError,
};

pub(crate) struct KeyStore {
    path: PathBuf,
    master_key: Arc<Crypter>,
    keys: Mutex<HashMap<TaskStatus, Arc<Crypter>>>,
}

// TODO: Clear poisoned mutex when panic happens.

impl KeyStore {
    pub(crate) fn load(path: &Path, crypther: Arc<Crypter>) -> Result<Arc<KeyStore>, RustError> {
        if !path.exists() {
            return Ok(Self {
                path: path.to_path_buf(),
                master_key: crypther,
                keys: Mutex::default(),
            }
            .into());
        }

        let mut keys = HashMap::new();

        let file = BufReader::new(File::open(path)?);
        for line in file.lines() {
            let line = line?;

            let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(line)?;

            let (aad, _iv, decrypted) = crypther.decrypt(&bytes, 1)?;

            let status = match aad[0] {
                b'p' => TaskStatus::Pending,
                b'c' => TaskStatus::Complete,
                b'd' => TaskStatus::Deleted,
                b'w' => TaskStatus::Waiting,
                b'r' => TaskStatus::Recurring,
                identifier => return Err(KeyStoreError::InvalidTaskStatus { identifier }.into()),
            };

            if decrypted.len() != 32 {
                return Err(KeyStoreError::InvalidCipherLength {
                    actual_length: decrypted.len(),
                    expected_length: 32,
                }
                .into());
            }

            if keys.contains_key(&status) {
                return Err(KeyStoreError::DuplicateEntry { status }.into());
            }

            keys.insert(
                status,
                Crypter::new(decrypted.try_into().expect("already checked correct size")).into(),
            );
        }

        Ok(Self {
            path: path.to_path_buf(),
            master_key: crypther,
            keys: Mutex::new(keys),
        }
        .into())
    }

    pub(crate) fn save(&self) -> Result<(), RustError> {
        let mut contents = String::new();
        let keys = self.keys.lock().map_err(|_| KeyStoreError::LockError)?;
        for (status, key) in &*keys {
            let aad = match status {
                TaskStatus::Pending => b'p',
                TaskStatus::Complete => b'c',
                TaskStatus::Deleted => b'd',
                TaskStatus::Waiting => b'w',
                TaskStatus::Recurring => b'r',
            };

            let encrypted = self.master_key.encrypt(key.encryption_key(), &[aad])?;
            let base64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(encrypted);

            contents.push_str(&base64);
            contents.push('\n');
        }
        std::fs::write(&self.path, contents)?;
        Ok(())
    }

    pub(crate) fn has_key_for(&self, status: TaskStatus) -> bool {
        let keys = self
            .keys
            .lock()
            .map_err(|_| anyhow::Error::msg("could not lock key store"))
            .unwrap();
        keys.contains_key(&status)
    }

    pub(crate) fn encrypt(
        &self,
        task: &Task,
        iv: Option<[u8; 12]>,
    ) -> Result<([u8; 12], String), RustError> {
        let mut keys = self.keys.lock().map_err(|_| KeyStoreError::LockError)?;
        let mut need_to_save = false;
        let key = if let Some(key) = keys.get(&task.status) {
            key.clone()
        } else {
            let key = Arc::new(Crypter::generate());
            keys.insert(task.status, key.clone());
            need_to_save = true;
            key
        };

        let data = task.to_data();

        let iv = iv.unwrap_or_else(generate_iv);
        let encrypted = key.encrypt_with_iv(&iv, &data[16..], task.uuid.as_bytes())?;
        let base64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(encrypted);

        drop(keys);

        if need_to_save {
            self.save()?;
        }
        Ok((iv, base64))
    }

    pub(crate) fn decrypt(
        &self,
        status: TaskStatus,
        base64: &str,
    ) -> Result<([u8; 12], Task), RustError> {
        let base64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(base64.trim())?;

        let keys = self.keys.lock().map_err(|_| KeyStoreError::LockError)?;
        let key = if let Some(key) = keys.get(&status) {
            key.clone()
        } else {
            let mut task = Task::from_data(&base64).ok_or(ErrorKind::CorruptTask)?;
            task.status = status;
            return Ok((generate_iv(), task));
        };

        let (aad, iv, decrypted) = key.decrypt(&base64, Uuid::max().as_bytes().len())?;

        let mut data = aad.to_vec();
        data.extend_from_slice(&decrypted);

        let mut task = Task::from_data(&data).ok_or(ErrorKind::CorruptTask)?;
        task.status = status;
        Ok((iv, task))
    }
}
