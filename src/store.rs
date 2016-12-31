pub trait ValueStore<K, V> {
    fn get(&mut self, key: &K) -> Option<&V>;
    fn set(&mut self, key: K, value: V);
}

pub trait MetadataStore<K, V, M> {
    fn get_meta(&self, key: K) -> Option<&M>;
    fn update_meta_on_get(&mut self, key: &K);
    fn update_meta_on_set(&mut self, key: &K, value: &V);
}

pub struct Store<K, V, M,
                 KVStore: ValueStore<K, V>,
                 KMStore: MetadataStore<K, V, M>>
{
    value_store: KVStore,
    metadata_store: KMStore,
    // Terrible hack to allow this struct to take the ValueStore and
    // MetadataStore traits and constrain them to having the same types for the
    // keys (K) and the values (V).
    // TODO: find a better way to achieve this
    __hack1: Option<K>,
    __hack2: Option<V>,
    __hack3: Option<M>,
}

impl<K, V, M, KVStore: ValueStore<K, V>, KMStore: MetadataStore<K, V, M>>
Store<K, V, M, KVStore, KMStore> {

    pub fn new(value_store: KVStore, metadata_store: KMStore)
    -> Store<K, V, M, KVStore, KMStore> {
        Store {
            value_store: value_store,
            metadata_store: metadata_store,
            __hack1: None,
            __hack2: None,
            __hack3: None,
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