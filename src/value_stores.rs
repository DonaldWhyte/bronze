use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use store::StoreStats;
use store::ValueStore;


pub struct HashMapValueStore<K: Eq + Hash, V> {
    data: HashMap<K, V>,
    stats: StoreStats,
}

impl<K: Eq + Hash, V> HashMapValueStore<K, V> {
    pub fn new() -> HashMapValueStore<K, V> {
        HashMapValueStore {
            data: HashMap::new(),
            stats: StoreStats::new()
        }
    }
}

impl<K: Eq + Hash, V> ValueStore<K, V> for HashMapValueStore<K, V> {

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(&key)
    }

    fn set(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    fn delete(&mut self, key: &K) -> bool {
        match self.data.remove(key) {
            Some(_) => true,
            None => false,
        }
    }

    fn delete_many(&mut self, keys: &Vec<K>) {
        for key in keys {
            self.delete(key);
        }
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn stats(&self) -> &StoreStats {
        &self.stats
    }

}
