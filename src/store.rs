pub struct StoreStats {
    num_keys: u64,
    total_memory_in_bytes: u64,
}

impl StoreStats {
    pub fn new() -> StoreStats {
        StoreStats {
            num_keys: 0,
            total_memory_in_bytes: 0,
        }
    }
}

pub trait ValueStore<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn set(&mut self, key: K, value: V);
    fn delete(&mut self, key: &K) -> bool;
    fn delete_many(&mut self, keys: &Vec<K>);
    fn clear(&mut self);
    fn stats(&self) -> &StoreStats;
}

pub trait MetadataStore<K, V> {
    fn update_metadata_on_get(&mut self, key: &K);
    fn update_metadata_on_set(&mut self, key: &K, value: &V);
    fn delete_many(&mut self, keys: &Vec<K>);
    fn clear(&mut self);
}

pub trait EvictionPolicy<K, V> {
    fn should_evict_keys(&self, store_stats: &StoreStats) -> bool;
    fn choose_keys_to_evict(&self) -> Vec<K>;
}

pub struct Store<K, V,
                 KVStore: ValueStore<K, V>,
                 KMStore: MetadataStore<K, V>,
                 EvPolicy: EvictionPolicy<K, V>>
{
    value_store: KVStore,
    metadata_store: KMStore,
    eviction_policy: EvPolicy,
    // Terrible hack to allow this struct to take the ValueStore and
    // MetadataStore traits and constrain them to having the same types for the
    // keys (K) and the values (V).
    // TODO: find a better way to achieve this
    __hack1: Option<K>,
    __hack2: Option<V>,
}

impl<K, V,
     KVStore: ValueStore<K, V>,
     KMStore: MetadataStore<K, V>,
     EvPolicy: EvictionPolicy<K, V>>
Store<K, V, KVStore, KMStore, EvPolicy>
{

    pub fn new(value_store: KVStore,
               metadata_store: KMStore,
               eviction_policy: EvPolicy) -> Store<K, V, KVStore, KMStore, EvPolicy>
    {
        Store {
            value_store: value_store,
            metadata_store: metadata_store,
            eviction_policy: eviction_policy,
            __hack1: None,
            __hack2: None,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.evict_keys_if_necessary();
        self.metadata_store.update_metadata_on_get(key);
        self.value_store.get(key)
    }

    pub fn set(&mut self, key: K, value: V) {
        self.evict_keys_if_necessary();
        self.metadata_store.update_metadata_on_set(&key, &value);
        self.value_store.set(key, value);
    }

    fn evict_keys_if_necessary(&mut self) {
        let should_evict = self.eviction_policy.should_evict_keys(
            self.value_store.stats());
        if should_evict {
            let keys_to_evict = self.eviction_policy.choose_keys_to_evict();
            self.value_store.delete_many(&keys_to_evict);
            self.metadata_store.delete_many(&keys_to_evict);
        }
    }

}