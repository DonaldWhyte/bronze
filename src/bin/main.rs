extern crate bronze;
extern crate time;
use bronze::store::Store;
use bronze::value_stores::HashMapValueStore;
use bronze::metadata_stores::HashMapMetadataStore;


fn main() {
    let value_store : HashMapValueStore<i32, i32> = HashMapValueStore::new();
    let metadata_store : HashMapMetadataStore<i32, i32> =
        HashMapMetadataStore::new();
    let mut store = Store::new(value_store, metadata_store);

    store.set(5, 32);

    match store.get(&5) {
        Some(value) => println!("Key found: {}", value),
        None => println!("Key not found")
    }
}