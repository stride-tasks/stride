use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct CacheEntry<T> {
    pub(crate) id: Uuid,
    pub(crate) value: Arc<T>,
    pub(crate) last_used: Instant,
}

#[derive(Debug)]
pub(crate) struct Cache<T> {
    pub(crate) ttl: Duration,
    pub(crate) entries: Vec<CacheEntry<T>>,
}

impl<T> Cache<T> {
    pub(crate) fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            entries: Vec::new(),
        }
    }

    pub(crate) fn evict_expired(&mut self, now: Instant) {
        self.entries.retain(|entry| {
            let idle_for = now.duration_since(entry.last_used);
            idle_for < self.ttl || Arc::strong_count(&entry.value) > 0
        });
    }

    pub(crate) fn get_mut(&mut self, id: Uuid) -> Option<&mut CacheEntry<T>> {
        self.entries.iter_mut().find(|entry| entry.id == id)
    }

    pub(crate) fn insert(&mut self, id: Uuid, value: Arc<T>) {
        self.entries.push(CacheEntry {
            id,
            value,
            last_used: Instant::now(),
        });
    }
}
