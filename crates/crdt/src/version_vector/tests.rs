use uuid::Uuid;

use crate::{
    actor::{Actor, ActorId},
    change::Sequence,
    hlc::{Microsecond, Timestamp},
    version_vector::{ChangeLocation, ChangeRange, VersionDifference, VersionVector},
};

fn create_version_difference(diffs: &[(ActorId, ChangeRange)]) -> VersionDifference {
    VersionDifference {
        map: diffs.iter().copied().collect(),
    }
}

#[test]
fn merge_two_emtpy_version_vectors() {
    let local = VersionVector::default();
    let remote = VersionVector::default();

    let diff = local.merge(&remote);
    assert_eq!(diff, create_version_difference(&[]));
}

#[test]
fn merge_exclusive_local_actor_with_zero_sequence() {
    let mut local = VersionVector::default();
    let remote = VersionVector::default();

    let actor_id = ActorId::new(Uuid::nil());
    let actor = Actor {
        id: actor_id,
        sequence: Sequence::new(0),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    local.insert(actor);

    let diff = local.merge(&remote);
    assert_eq!(diff, create_version_difference(&[]));
}

#[test]
fn merge_exclusive_remote_actor_with_zero_sequence() {
    let local = VersionVector::default();
    let mut remote = VersionVector::default();

    let actor_id = ActorId::new(Uuid::nil());
    let actor = Actor {
        id: actor_id,
        sequence: Sequence::new(0),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    remote.insert(actor);

    let diff = local.merge(&remote);
    assert_eq!(diff, create_version_difference(&[]));
}

#[test]
fn merge_exclusive_remote_actor_with_non_zero_sequence() {
    let local = VersionVector::default();
    let mut remote = VersionVector::default();

    let actor_id = ActorId::new(Uuid::nil());
    let actor = Actor {
        id: actor_id,
        sequence: Sequence::new(10),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    remote.insert(actor);

    let diff = local.merge(&remote);
    assert_eq!(
        diff,
        create_version_difference(&[(
            actor_id,
            ChangeRange {
                location: ChangeLocation::Remote,
                from: 0,
                count: 10
            }
        )])
    );
}

#[test]
fn merge_exclusive_local_actor_with_non_zero_sequence() {
    let mut local = VersionVector::default();
    let remote = VersionVector::default();

    let actor_id = ActorId::new(Uuid::nil());
    let actor = Actor {
        id: actor_id,
        sequence: Sequence::new(5),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    local.insert(actor);

    let diff = local.merge(&remote);
    assert_eq!(
        diff,
        create_version_difference(&[(
            actor_id,
            ChangeRange {
                location: ChangeLocation::Local,
                from: 0,
                count: 5
            }
        )])
    );
}

#[test]
fn merge_multiple_exclusive_local_actor_with_non_zero_sequence() {
    let mut local = VersionVector::default();
    let remote = VersionVector::default();

    let actor_id_a = ActorId::new(Uuid::max());
    let actor_a = Actor {
        id: actor_id_a,
        sequence: Sequence::new(5),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    let actor_id_b = ActorId::new(Uuid::nil());
    let actor_b = Actor {
        id: actor_id_b,
        sequence: Sequence::new(3),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    local.insert(actor_a);
    local.insert(actor_b);

    let diff = local.merge(&remote);
    assert_eq!(
        diff,
        create_version_difference(&[
            (
                actor_id_a,
                ChangeRange {
                    location: ChangeLocation::Local,
                    from: 0,
                    count: 5
                }
            ),
            (
                actor_id_b,
                ChangeRange {
                    location: ChangeLocation::Local,
                    from: 0,
                    count: 3
                }
            )
        ])
    );
}

#[test]
fn merge_exclusive_local_and_remote_actors_with_non_zero_sequence() {
    let mut local = VersionVector::default();
    let mut remote = VersionVector::default();

    let actor_id_a = ActorId::new(Uuid::max());
    let actor_a = Actor {
        id: actor_id_a,
        sequence: Sequence::new(5),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    let actor_id_b = ActorId::new(Uuid::nil());
    let actor_b = Actor {
        id: actor_id_b,
        sequence: Sequence::new(3),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    local.insert(actor_a);
    remote.insert(actor_b);

    let diff = local.merge(&remote);
    assert_eq!(
        diff,
        create_version_difference(&[
            (
                actor_id_a,
                ChangeRange {
                    location: ChangeLocation::Local,
                    from: 0,
                    count: 5
                }
            ),
            (
                actor_id_b,
                ChangeRange {
                    location: ChangeLocation::Remote,
                    from: 0,
                    count: 3
                }
            )
        ])
    );
}

#[test]
fn merge_non_exclusive_actor_advanced_local() {
    let mut local = VersionVector::default();
    let mut remote = VersionVector::default();

    let actor_id = ActorId::new(Uuid::max());
    let actor_a = Actor {
        id: actor_id,
        sequence: Sequence::new(5),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    let actor_b = Actor {
        id: actor_id,
        sequence: Sequence::new(3),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    local.insert(actor_a);
    remote.insert(actor_b);

    let diff = local.merge(&remote);
    assert_eq!(
        diff,
        create_version_difference(&[(
            actor_id,
            ChangeRange {
                location: ChangeLocation::Local,
                from: 3,
                count: 2
            }
        )])
    );
}

#[test]
fn merge_non_exclusive_actor_advanced_remote() {
    let mut local = VersionVector::default();
    let mut remote = VersionVector::default();

    let actor_id = ActorId::new(Uuid::max());
    let actor_local = Actor {
        id: actor_id,
        sequence: Sequence::new(3),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    let actor_remote = Actor {
        id: actor_id,
        sequence: Sequence::new(5),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    local.insert(actor_local);
    remote.insert(actor_remote);

    let diff = local.merge(&remote);
    assert_eq!(
        diff,
        create_version_difference(&[(
            actor_id,
            ChangeRange {
                location: ChangeLocation::Remote,
                from: 3,
                count: 2
            }
        )])
    );
}

#[test]
fn merge_non_exclusive_actor_same_local_and_remote() {
    let mut local = VersionVector::default();
    let mut remote = VersionVector::default();

    let actor_id = ActorId::new(Uuid::max());
    let actor_a = Actor {
        id: actor_id,
        sequence: Sequence::new(5),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    let actor_b = Actor {
        id: actor_id,
        sequence: Sequence::new(5),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    local.insert(actor_a);
    remote.insert(actor_b);

    let diff = local.merge(&remote);
    assert_eq!(diff, create_version_difference(&[]));
}

#[test]
fn merge_mixed_actors() {
    let mut local = VersionVector::default();
    let mut remote = VersionVector::default();

    let actor_id_a = ActorId::new(Uuid::nil());
    let actor_a_local = Actor {
        id: actor_id_a,
        sequence: Sequence::new(5),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };
    let actor_a_remote = Actor {
        id: actor_id_a,
        sequence: Sequence::new(9),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };

    let actor_id_b = ActorId::new(Uuid::max());
    let actor_b = Actor {
        id: actor_id_b,
        sequence: Sequence::new(10),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };

    let actor_id_c = ActorId::new(Uuid::from_u128(0xFFFF_0000_FFFF));
    let actor_c = Actor {
        id: actor_id_c,
        sequence: Sequence::new(20),
        timestamp: Timestamp::new(Microsecond::new(0), 0),
    };

    local.insert(actor_a_local);
    local.insert(actor_b);
    remote.insert(actor_a_remote);
    remote.insert(actor_c);

    let diff = local.merge(&remote);
    assert_eq!(
        diff,
        create_version_difference(&[
            (
                actor_id_a,
                ChangeRange {
                    location: ChangeLocation::Remote,
                    from: 5,
                    count: 4
                }
            ),
            (
                actor_id_b,
                ChangeRange {
                    location: ChangeLocation::Local,
                    from: 0,
                    count: 10
                }
            ),
            (
                actor_id_c,
                ChangeRange {
                    location: ChangeLocation::Remote,
                    from: 0,
                    count: 20
                }
            )
        ])
    );
}
