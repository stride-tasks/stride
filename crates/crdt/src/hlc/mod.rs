use chrono::Utc;
use std::{fmt::Display, sync::Arc};

use crate::{Error, Result};

#[cfg(test)]
mod tests;

/// Milliseconds since UNIX epoch.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Microsecond(i64);

impl Display for Microsecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}us", self.0)
    }
}

impl Microsecond {
    #[must_use]
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn from_seconds(value: i64) -> Self {
        Self::new(value * 1000 * 1000)
    }

    #[must_use]
    pub fn from_minutes(value: i64) -> Self {
        Self::from_seconds(value * 60)
    }

    #[must_use]
    pub fn get(self) -> i64 {
        self.0
    }
}

impl std::ops::Add<Microsecond> for Microsecond {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}
impl std::ops::Sub<Microsecond> for Microsecond {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}
impl std::ops::Neg for Microsecond {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.0)
    }
}

pub trait TimeProvider: std::fmt::Debug {
    fn now(&self) -> Microsecond;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemTimeProvider {}

impl TimeProvider for SystemTimeProvider {
    fn now(&self) -> Microsecond {
        Microsecond::new(Utc::now().timestamp_micros())
    }
}

pub type Counter = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    pub logical: Microsecond,
    pub counter: Counter,
}

impl Timestamp {
    pub const MAX_COUNTER: Counter = Counter::MAX;

    #[must_use]
    pub fn new(logical: Microsecond, counter: Counter) -> Self {
        Self { logical, counter }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Max allowed clock drift.
    pub max_drift: Microsecond,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_drift: Microsecond::from_minutes(5),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Clock {
    time_provider: Arc<dyn TimeProvider + Sync + Send>,
    timestamp: Timestamp,
    config: Config,
}

impl Clock {
    #[must_use]
    pub fn with_config(time_provider: Arc<dyn TimeProvider + Sync + Send>, config: Config) -> Self {
        Self {
            timestamp: Timestamp {
                logical: time_provider.now(),
                counter: 0,
            },
            time_provider,
            config,
        }
    }

    #[must_use]
    pub fn new(time_provider: Arc<dyn TimeProvider + Sync + Send>) -> Self {
        Self::with_config(time_provider, Config::default())
    }

    #[must_use]
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    #[must_use]
    pub fn time_provider(&self) -> &Arc<dyn TimeProvider + Sync + Send> {
        &self.time_provider
    }

    /// Generates a unique, monotonic [`Timestamp`] suitable for transmission to another system.
    ///
    /// # Errors
    ///
    /// Fails if timestamp counter overflows or the clock drift is greater than [`Config::max_drift`].
    pub fn tick(&mut self) -> Result<Timestamp> {
        let physical = self.time_provider.now();

        let logical_new = self.timestamp.logical.max(physical);
        let counter_new = if self.timestamp.logical == logical_new {
            self.timestamp
                .counter
                .checked_add(1)
                .ok_or(Error::TimestampCounterOverflow)?
        } else {
            0
        };

        if logical_new - physical > self.config.max_drift {
            return Err(Error::ClockDrift {
                local: true,
                logical: logical_new,
                physical,
                max_drift: self.config.max_drift,
            });
        }

        self.timestamp = Timestamp::new(logical_new, counter_new);

        Ok(self.timestamp)
    }

    /// Merges a [`Timestamp`] from a remote system with the local time,
    /// preserving uniqueness and monotonicity.
    ///
    /// # Errors
    ///
    /// Fails if timestamp counter overflows or the clock drift is greater than [`Config::max_drift`].
    pub fn merge(&mut self, remote: Timestamp) -> Result<Timestamp> {
        let physical = self.time_provider.now();

        if remote.logical - physical > self.config.max_drift {
            return Err(Error::ClockDrift {
                local: false,
                logical: remote.logical,
                physical,
                max_drift: self.config.max_drift,
            });
        }

        let logical = self.timestamp.logical.max(remote.logical).max(physical);
        if logical - physical > self.config.max_drift {
            return Err(Error::ClockDrift {
                local: true,
                logical,
                physical,
                max_drift: self.config.max_drift,
            });
        }

        let counter = if logical == self.timestamp.logical && logical == remote.logical {
            self.timestamp.counter.max(remote.counter).checked_add(1)
        } else if logical == self.timestamp.logical {
            self.timestamp.counter.checked_add(1)
        } else if logical == remote.logical {
            remote.counter.checked_add(1)
        } else {
            Some(0)
        }
        .ok_or(Error::TimestampCounterOverflow)?;

        self.timestamp = Timestamp::new(logical, counter);

        Ok(self.timestamp)
    }
}

#[cfg(feature = "serialize")]
mod serialize {
    use crate::hlc::{Microsecond, Timestamp};

    impl stride_serialize::ToBlob<'_> for Microsecond {
        fn to_blob(&self, blob: &mut Vec<u8>) {
            self.get().to_blob(blob);
        }
    }

    impl stride_serialize::FromBlob<'_> for Microsecond {
        fn from_blob(blob: &mut &[u8]) -> stride_serialize::Result<Self> {
            Ok(Self::new(i64::from_blob(blob)?))
        }
    }

    impl stride_serialize::ToBlob<'_> for Timestamp {
        fn to_blob(&self, blob: &mut Vec<u8>) {
            self.logical.to_blob(blob);
            self.counter.to_blob(blob);
        }
    }

    impl stride_serialize::FromBlob<'_> for Timestamp {
        fn from_blob(blob: &mut &[u8]) -> stride_serialize::Result<Self> {
            Ok(Timestamp::new(
                Microsecond::from_blob(blob)?,
                u32::from_blob(blob)?,
            ))
        }
    }
}
