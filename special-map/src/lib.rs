use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash};

#[derive(Debug)]
pub struct BidirRemovableMap<K, V, S = RandomState>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher,
{
    set: HashSet<K, S>,
    map: HashMap<V, K, S>,
}

impl<K, V, S> BidirRemovableMap<K, V, S>
where
    K: Hash + Eq + Clone,
    V: Hash + Eq,
    S: BuildHasher,
{
    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.set.contains(key)
    }

    pub fn insert(&mut self, key: K, val: V) {
        self.set.insert(key.clone());
        self.map.insert(val, key);
    }

    pub fn remove_by_value(&mut self, val: &V) -> Option<K> {
        let key = self.map.remove(val);

        if let Some(ref key) = key {
            self.set.remove(key);
        }

        key
    }
}

impl<K, V, S> Default for BidirRemovableMap<K, V, S>
where
    K: Eq + Hash,
    V: Eq + Hash,
    S: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            set: Default::default(),
            map: Default::default(),
        }
    }
}
