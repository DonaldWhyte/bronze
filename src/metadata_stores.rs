use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use store::MetadataStore;


pub struct HashMapMetadataStore<K: Eq + Hash, M> {
    metadata: HashMap<K, M>,
}

impl<K: Eq + Hash, V> HashMapMetadataStore<K, V> {
    pub fn new() -> HashMapMetadataStore<K, V> {
        HashMapMetadataStore {
            metadata: HashMap::new()
        }
    }
}

impl<K: Eq + Hash, V, M> MetadataStore<K, V, M> for HashMapMetadataStore<K, M> {

    fn get_meta(&self, key: K) -> Option<&M> {
        self.metadata.get(&key)
    }

    fn update_meta_on_get(&mut self, key: &K) {
        // TODO
    }

    fn update_meta_on_set(&mut self, key: &K, value: &V) {
        // TODO
    }

}
