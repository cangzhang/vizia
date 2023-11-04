use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use crate::{model::ModelOrView, prelude::*};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoreId(pub u64);

pub(crate) fn get_storeid<T: Hash>(t: &T) -> StoreId {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    StoreId(s.finish())
}

pub(crate) trait Store {
    /// Updates the model data, returning true if the data changed.
    fn update(&mut self, model: ModelOrView) -> bool;
    /// Returns the set of observers for the store.
    fn observers(&self) -> &HashSet<Entity>;
    /// Adds an observer to the store.
    fn add_observer(&mut self, observer: Entity);
    /// Removes an observer from the store.
    fn remove_observer(&mut self, observer: &Entity);
    /// Returns the number of obersers for the store.
    fn num_observers(&self) -> usize;
    fn contains_source(&self, model: ModelOrView) -> bool;

    fn name(&self) -> String;
}

pub(crate) struct BasicStore<L: Lens, T> {
    pub lens: L,
    pub old: Option<T>,
    pub observers: HashSet<Entity>,
}

impl<L: Lens, T> Store for BasicStore<L, T>
where
    L: Lens<Target = T>,
    <L as Lens>::Target: Data,
{
    fn contains_source(&self, model: ModelOrView) -> bool {
        model.downcast_ref::<L::Source>().is_some()
    }
    fn update(&mut self, model: ModelOrView) -> bool {
        let Some(data) = model.downcast_ref::<L::Source>() else { return false };
        let new_data = self.lens.view(data);

        if matches!(&self.old, Some(old) if old.same(&new_data)) {
            return false;
        }

        self.old = Some(new_data.deref().clone());

        true
    }

    fn observers(&self) -> &HashSet<Entity> {
        &self.observers
    }

    fn add_observer(&mut self, observer: Entity) {
        self.observers.insert(observer);
    }

    fn remove_observer(&mut self, observer: &Entity) {
        self.observers.remove(observer);
    }

    fn num_observers(&self) -> usize {
        self.observers.len()
    }

    fn name(&self) -> String {
        format!("{:?}", self.lens)
    }
}
