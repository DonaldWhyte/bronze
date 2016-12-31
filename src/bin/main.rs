extern crate bronze;
extern crate rand;
extern crate time;
use bronze::store::Store;
use bronze::value_stores::HashMapValueStore;
use bronze::lfu::LfuEvictionPolicy;

type Key = u32;
type Value = u32;

fn main() {
    let value_store : HashMapValueStore<Key, Value> = HashMapValueStore::new();

    let start_time_in_epoch_seconds = time::now_utc().to_timespec().sec;
    let max_memory_in_bytes = 16;
    let sample_size = 5;
    let decay_time_in_mins = 1;
    let eviction_policy : LfuEvictionPolicy<Key, _> = LfuEvictionPolicy::new(
        rand::thread_rng(),
        start_time_in_epoch_seconds,
        max_memory_in_bytes,
        sample_size,
        decay_time_in_mins);

    let mut store = Store::new(value_store, eviction_policy);

    store.set(5, 32);

    for i in 1..500000 {
    match store.get(&5) {
        Some(value) => println!("Key found: {}", value),
        None => println!("Key not found")
    }
    }
    match store.get(&6) {
        Some(value) => println!("Key found: {}", value),
        None => println!("Key not found")
    }
}