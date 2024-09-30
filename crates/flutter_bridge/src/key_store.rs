use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc, Mutex},
};
use stride_crypto::crypter::{Aes256Ocb, AesMode, Crypter};
use uuid::Uuid;

use crate::{
    api::{error::KeyStoreError, repository::generate_iv},
    base64_decode, base64_encode,
    task::{Task, TaskStatus},
    ErrorKind, RustError,
};

pub(crate) struct KeyStore {
    path: PathBuf,
    master_key: Arc<Crypter>,
    keys: Mutex<HashMap<TaskStatus, Arc<Crypter>>>,
    loaded: AtomicBool,
}

// TODO: Clear poisoned mutex when panic happens.

impl KeyStore {
    pub(crate) fn new(path: &Path, crypther: Arc<Crypter>) -> Self {
        Self {
            path: path.to_path_buf(),
            keys: HashMap::new().into(),
            loaded: false.into(),
            master_key: crypther,
        }
    }
    pub(crate) fn load(&self) -> Result<(), RustError> {
        if self.loaded.load(std::sync::atomic::Ordering::SeqCst) {
            return Ok(());
        }
        if !self.path.exists() {
            return Ok(());
        }

        let mut keys = HashMap::new();

        let file = BufReader::new(File::open(&self.path)?);
        for line in file.lines() {
            let line = line?;

            let bytes = base64_decode(line)?;

            let (aad, _iv, decrypted) = self.master_key.decrypt(&bytes, 1)?;

            let status = match aad[0] {
                b'p' => TaskStatus::Pending,
                b'c' => TaskStatus::Complete,
                b'd' => TaskStatus::Deleted,
                b'w' => TaskStatus::Waiting,
                b'r' => TaskStatus::Recurring,
                identifier => return Err(KeyStoreError::InvalidTaskStatus { identifier }.into()),
            };

            let Ok(decrypted) = decrypted.as_slice().try_into() else {
                return Err(KeyStoreError::InvalidCipherLength {
                    actual_length: decrypted.len(),
                    expected_length: Aes256Ocb::KEY_LEN,
                }
                .into());
            };

            if keys.contains_key(&status) {
                return Err(KeyStoreError::DuplicateEntry { status }.into());
            }

            keys.insert(status, Crypter::new(decrypted).into());
        }

        *self.keys.lock().map_err(|_| KeyStoreError::LockError)? = keys;
        self.loaded.store(true, std::sync::atomic::Ordering::SeqCst);

        Ok(())
    }

    pub(crate) fn save(&self) -> Result<(), RustError> {
        self.load()?;
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
            let base64 = base64_encode(encrypted);

            contents.push_str(&base64);
            contents.push('\n');
        }
        std::fs::write(&self.path, contents)?;
        Ok(())
    }

    pub(crate) fn has_key_for(&self, status: TaskStatus) -> Result<bool, RustError> {
        self.load()?;
        let keys = self.keys.lock().map_err(|_| KeyStoreError::LockError)?;
        Ok(keys.contains_key(&status))
    }

    pub(crate) fn encrypt(
        &self,
        task: &Task,
        iv: Option<[u8; 12]>,
    ) -> Result<([u8; 12], String), RustError> {
        self.load()?;

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
        let base64 = base64_encode(encrypted);

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
        self.load()?;

        let base64 = base64_decode(base64.trim())?;

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
