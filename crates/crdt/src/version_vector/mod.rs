use std::collections::BTreeMap;

use crate::{
    Error, Result,
    actor::{Actor, ActorId},
    change::{Change, Sequence},
};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChangeLocation {
    Local,
    Remote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChangeRange {
    pub location: ChangeLocation,
    pub from: u64,
    pub count: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionDifference {
    map: BTreeMap<ActorId, ChangeRange>,
}

impl IntoIterator for VersionDifference {
    type Item = (ActorId, ChangeRange);
    type IntoIter = <BTreeMap<ActorId, ChangeRange> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

#[derive(Debug, Default, Clone)]
pub struct VersionVector {
    map: BTreeMap<ActorId, Actor>,
}

impl VersionVector {
    #[must_use]
    pub fn contains(&self, actor_id: ActorId) -> bool {
        self.map.contains_key(&actor_id)
    }

    pub fn insert(&mut self, actor: Actor) {
        self.map.insert(actor.id, actor);
    }

    #[must_use]
    pub fn merge(&self, remote: &Self) -> VersionDifference {
        let mut map = BTreeMap::new();
        for actor in self.map.values() {
            let from = actor.sequence.get();
            if let Some(remote_actor) = remote.map.get(&actor.id) {
                let to = remote_actor.sequence.get();
                let count = from.cast_signed().abs_diff(to.cast_signed());

                if count == 0 {
                    continue;
                }

                let (from, location) = if from > to {
                    (to, ChangeLocation::Local)
                } else {
                    (from, ChangeLocation::Remote)
                };

                map.insert(
                    actor.id,
                    ChangeRange {
                        location,
                        from,
                        count,
                    },
                );
            } else {
                let count = from;
                if count == 0 {
                    continue;
                }
                map.insert(
                    actor.id,
                    ChangeRange {
                        location: ChangeLocation::Local,
                        from: 0,
                        count,
                    },
                );
            }
        }

        for actor in remote.map.values() {
            if !self.map.contains_key(&actor.id) {
                let count = actor.sequence.get();
                if count == 0 {
                    continue;
                }
                map.insert(
                    actor.id,
                    ChangeRange {
                        location: ChangeLocation::Remote,
                        from: 0,
                        count,
                    },
                );
            }
        }

        VersionDifference { map }
    }

    /// Get a reference to the actor.
    ///
    /// # Errors
    ///
    /// Fails if actor cannot be found ([`Error::MissingActor`]).
    pub fn get(&self, actor_id: ActorId) -> Result<&Actor> {
        self.map
            .get(&actor_id)
            .ok_or(Error::MissingActor { actor_id })
    }

    /// Returns the next sequence for a specified [`ActorId`].
    ///
    /// # Errors
    ///
    /// Will fail with [`Error::MissingActor`] if actor could not be found.
    pub fn next_sequence(&mut self, actor_id: ActorId) -> Result<Sequence> {
        let actor = self.get(actor_id)?;
        Ok(Sequence::new(actor.sequence.get() + 1))
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn apply(&mut self, change: &Change) -> Result<Sequence> {
        let Some(actor) = self.map.get_mut(&change.actor_id) else {
            return Err(Error::MissingActor {
                actor_id: change.actor_id,
            });
        };

        if change.sequence.get() <= actor.sequence.get() {
            return Err(Error::ReapplyingChange {
                actor_id: change.actor_id,
                actor_sequence: actor.sequence.get(),
                change_sequence: change.sequence.get(),
            });
        }

        if actor.sequence.get() + 1 != change.sequence.get() {
            return Err(Error::MissingChangeSequence {
                actor_id: change.actor_id,
                from: actor.sequence.get(),
                to: change.sequence.get(),
            });
        }
        actor.sequence = change.sequence;
        actor.timestamp = change.timestamp;
        Ok(actor.sequence)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Actor> {
        self.map.values()
    }
}
