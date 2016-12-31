use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use store::EvictionPolicy;
use store::MetadataStore;
use store::StoreStats;

// MetadataStore

struct LfuMetadata {
    useFrequency: u8,
    lastUsedTimestamp: u16,
}

pub struct LfuMetadataStore<K: Eq + Hash> {
    metadata: HashMap<K, LfuMetadata>,
}

impl<K: Eq + Hash> LfuMetadataStore<K> {
    pub fn new() -> LfuMetadataStore<K> {
        LfuMetadataStore {
            metadata: HashMap::new()
        }
    }
}

impl<K: Eq + Hash, V> MetadataStore<K, V> for LfuMetadataStore<K> {
    fn update_metadata_on_get(&mut self, key: &K) {
        // TODO
    }

    fn update_metadata_on_set(&mut self, key: &K, value: &V) {
        // TODO
    }

    fn delete_many(&mut self, keys: &Vec<K>) {
        for key in keys {
            self.metadata.remove(key);
        }
    }

    fn clear(&mut self) {
        self.metadata.clear();
    }
}

// EvictionPolicy

pub struct LfuEvictionPolicy<K: Eq + Hash> {
    metadata_store: LfuMetadataStore<K>,
}

impl<K: Eq + Hash> LfuEvictionPolicy<K> {
    pub fn new() -> LfuEvictionPolicy<K> {
        LfuEvictionPolicy {
            metadata_store: LfuMetadataStore::new()
        }
    }
}

impl<K: Eq + Hash, V> EvictionPolicy<K, V> for LfuEvictionPolicy<K> {
    fn should_evict_keys(&self, store_stats: &StoreStats) -> bool {
        // TODO
        false
    }

    fn choose_keys_to_evict(&self) -> Vec<K> {
        // TODO
        Vec::new()
    }
}
