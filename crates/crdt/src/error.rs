use crate::{actor::ActorId, hlc::Microsecond};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    #[error(
        "clock drift(local: {local}): logical: {logical}, physical: {physical}, max_drift: {max_drift}"
    )]
    ClockDrift {
        local: bool,
        logical: Microsecond,
        physical: Microsecond,
        max_drift: Microsecond,
    },
    #[error("timestamp counter overflow")]
    TimestampCounterOverflow,
    #[error("missing actor {actor_id}")]
    MissingActor { actor_id: ActorId },
    #[error("missing change sequences [{from}..{to}] from actor id {actor_id}")]
    MissingChangeSequence {
        actor_id: ActorId,
        from: u64,
        to: u64,
    },
    #[error(
        "attempting to re-apply change with sequence {change_sequence} older than the current actor {actor_id} sequence {actor_sequence}"
    )]
    ReapplyingChange {
        actor_id: ActorId,
        actor_sequence: u64,
        change_sequence: u64,
    },
}
