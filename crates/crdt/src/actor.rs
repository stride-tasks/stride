use std::{fmt::Display, sync::Arc};

use uuid::Uuid;

use crate::{
    change::Sequence,
    hlc::{TimeProvider, Timestamp},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActorId(Uuid);

impl Display for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Default for ActorId {
    fn default() -> Self {
        Self::now()
    }
}

impl ActorId {
    #[must_use]
    pub fn now() -> Self {
        Self(Uuid::now_v7())
    }

    #[must_use]
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    #[must_use]
    pub fn get(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_copy_implementations)]
pub struct Actor {
    pub id: ActorId,
    pub sequence: Sequence,
    pub timestamp: Timestamp,
}

impl Actor {
    #[must_use]
    pub fn new(id: ActorId, timestamp: Timestamp) -> Self {
        Actor {
            id,
            sequence: Sequence::default(),
            timestamp,
        }
    }

    #[must_use]
    pub fn now(clock: &Arc<dyn TimeProvider + Sync + Send>) -> Self {
        Actor {
            id: ActorId::now(),
            sequence: Sequence::default(),
            timestamp: Timestamp::new(clock.now(), 0),
        }
    }
}

#[cfg(feature = "serialize")]
mod serialize {
    use uuid::Uuid;

    use crate::actor::ActorId;

    impl stride_serialize::ToBlob<'_> for ActorId {
        fn to_blob(&self, blob: &mut Vec<u8>) {
            self.get().to_blob(blob);
        }
    }

    impl stride_serialize::FromBlob<'_> for ActorId {
        fn from_blob(blob: &mut &[u8]) -> stride_serialize::Result<Self> {
            Ok(ActorId::new(Uuid::from_blob(blob)?))
        }
    }
}
