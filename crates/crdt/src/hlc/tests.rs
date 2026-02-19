use std::sync::{Arc, atomic::AtomicI64};

use crate::{
    Error,
    hlc::{Clock, Counter, Microsecond, TimeProvider, Timestamp},
};

#[derive(Debug)]
struct TestTimeProvider {
    time: AtomicI64,
}

impl TestTimeProvider {
    fn new(value: i64) -> Self {
        Self {
            time: AtomicI64::new(value),
        }
    }

    fn set(&self, value: i64) -> i64 {
        self.time.swap(value, std::sync::atomic::Ordering::SeqCst)
    }
    fn increment(&self) -> i64 {
        self.time.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
    fn increment_by(&self, value: i64) -> i64 {
        self.time
            .fetch_add(value, std::sync::atomic::Ordering::SeqCst)
    }
    fn decrement(&self) -> i64 {
        self.time.fetch_sub(1, std::sync::atomic::Ordering::SeqCst)
    }
    fn decrement_by(&self, value: i64) -> i64 {
        self.time
            .fetch_sub(value, std::sync::atomic::Ordering::SeqCst)
    }
}

impl TimeProvider for TestTimeProvider {
    fn now(&self) -> Microsecond {
        Microsecond(self.time.load(std::sync::atomic::Ordering::SeqCst))
    }
}

fn init() -> (Arc<TestTimeProvider>, Clock) {
    let time_provider = Arc::new(TestTimeProvider::new(0));
    (time_provider.clone(), Clock::new(time_provider))
}

// #[test]
// fn test_current_time_retrieval() {
//     let time_provider = Arc::new(TestTimeProvider::new(0));
//     let clock = Clock::new(time_provider.clone());

//     assert_eq!(clock.current_time(), 0);
//     time_provider.set(100);
//     assert_eq!(clock.current_time(), 100);
// }

#[test]
fn tick_monotonically_with_monotonic_clock() {
    let (provider, mut clock) = init();

    provider.set(100);
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(100), 0)
    );
    provider.increment();
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(101), 0)
    );
    provider.increment();
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(102), 0)
    );
    provider.increment_by(20);
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(122), 0)
    );
}

#[test]
fn tick_monotonically_with_stuttering_clock() {
    let (provider, mut clock) = init();

    provider.set(10);
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(10), 0)
    );
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(10), 1)
    );
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(10), 2)
    );
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(10), 3)
    );
    provider.increment_by(15);
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(25), 0)
    );
}

#[test]
fn tick_monotonically_with_regressing_clock() {
    let (provider, mut clock) = init();

    provider.set(100);
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(100), 0)
    );
    provider.decrement();
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(100), 1)
    );
    provider.decrement_by(100);
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(100), 2)
    );
    provider.set(150);
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(150), 0)
    );
}

#[test]
fn tick_counter_overflow() {
    let (provider, mut clock) = init();

    provider.set(100);
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(100), 0)
    );
    clock.timestamp.counter = Counter::MAX - 2;

    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(100), Counter::MAX - 1)
    );
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(100), Counter::MAX)
    );

    assert_eq!(clock.tick(), Err(Error::TimestampCounterOverflow));
    assert_eq!(clock.tick(), Err(Error::TimestampCounterOverflow));
}

#[test]
fn tick_clock_drift_error() {
    let (provider, mut clock) = init();

    provider.set(100);
    assert_eq!(
        clock.tick().unwrap(),
        Timestamp::new(Microsecond::new(100), 0)
    );

    let physical_timestamp = -(clock.config.max_drift + Microsecond::new(1));
    provider.set(physical_timestamp.get());
    assert_eq!(
        clock.tick(),
        Err(Error::ClockDrift {
            local: true,
            logical: Microsecond::new(100),
            physical: physical_timestamp,
            max_drift: clock.config.max_drift
        })
    );
}

#[test]
fn merge_monotonically_with_monotonic_clock() {
    let (provider, mut clock) = init();

    provider.set(100);
    assert_eq!(
        clock.merge(Timestamp::new(Microsecond::new(99), 0)),
        Ok(Timestamp::new(Microsecond::new(100), 0))
    );

    provider.set(104);
    assert_eq!(
        clock.merge(Timestamp::new(Microsecond::new(102), 0)),
        Ok(Timestamp::new(Microsecond::new(104), 0))
    );

    provider.set(115);
    assert_eq!(
        clock.merge(Timestamp::new(Microsecond::new(111), 0)),
        Ok(Timestamp::new(Microsecond::new(115), 0))
    );
}

#[test]
fn merge_monotonically_with_stuttering_global_clock() {
    let (provider, mut clock) = init();

    provider.set(100);
    assert_eq!(
        clock.merge(Timestamp::new(Microsecond::new(99), 0)),
        Ok(Timestamp::new(Microsecond::new(100), 0))
    );
    assert_eq!(
        clock.merge(Timestamp::new(Microsecond::new(120), 0)),
        Ok(Timestamp::new(Microsecond::new(120), 1))
    );
    assert_eq!(
        clock.merge(Timestamp::new(Microsecond::new(200), 0)),
        Ok(Timestamp::new(Microsecond::new(200), 1))
    );

    provider.set(215);
    assert_eq!(
        clock.merge(Timestamp::new(Microsecond::new(205), 0)),
        Ok(Timestamp::new(Microsecond::new(215), 0))
    );
}
