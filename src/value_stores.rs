use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use store::ValueStore;


pub struct HashMapValueStore<K: Eq + Hash, V> {
    data: HashMap<K, V>,
}

impl<K: Eq + Hash, V> HashMapValueStore<K, V> {
    pub fn new() -> HashMapValueStore<K, V> {
        HashMapValueStore {
            data: HashMap::new()
        }
    }
}

impl<K: Eq + Hash, V> ValueStore<K, V> for HashMapValueStore<K, V> {

    fn get(&mut self, key: &K) -> Option<&V> {
        self.data.get(&key)
    }

    fn set(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

}
