use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

// TODO: move protocols to a single file
// TODO: move hash map value store implementation to a single file
// TODO: move hash map metadata type (for LRU) implementation to a single file

// Protocols

pub trait ValueStore<K, V> {
    fn get(&mut self, key: &K) -> Option<&V>;
    fn set(&mut self, key: K, value: V);
}

pub trait MetadataStore<K, V, M> {
    fn get_meta(&self, key: K) -> Option<&M>;
    fn update_meta_on_get(&mut self, key: &K);
    fn update_meta_on_set(&mut self, key: &K, value: &V);
}

pub struct Store<K,
                 V,
                 M,
                 KVStore: ValueStore<K, V>,
                 KMStore: MetadataStore<K, V, M>>
{
    value_store: KVStore,
    metadata_store: KMStore,
    // Terrible hack to allow this struct to take the ValueStore and
    // MetadataStore traits and constrain them to having the same types for the
    // keys (K) and the values (V).
    // TODO: find a better way to achieve this
    _hack1: Option<K>,
    _hack2: Option<V>,
    _hack3: Option<M>,
}

impl<K, V, M, KVStore: ValueStore<K, V>, KMStore: MetadataStore<K, V, M>>
Store<K, V, M, KVStore, KMStore> {

    pub fn new(value_store: KVStore, metadata_store: KMStore)
    -> Store<K, V, M, KVStore, KMStore> {
        Store {
            value_store: value_store,
            metadata_store: metadata_store,
            _hack1: None,
            _hack2: None,
            _hack3: None,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.metadata_store.update_meta_on_get(key);
        self.value_store.get(key)
    }

    pub fn set(&mut self, key: K, value: V) {
        self.metadata_store.update_meta_on_set(&key, &value);
        self.value_store.set(key, value);
    }

    pub fn get_meta(&self, key: K) -> Option<&M> {
        self.metadata_store.get_meta(key)
    }

}

// Implementations below

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

pub fn hello_world() -> String {
    "Hello".to_string()
}