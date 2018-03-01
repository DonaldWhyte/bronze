extern crate rand;
extern crate time;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use store::EvictionPolicy;
use store::StoreStats;

const LFU_LOG_FACTOR: f64 = 10.0;
const INITIAL_USE_COUNTER: u8 = 5;
const FAST_COUNTER_DECREMENT_THRESHOLD: u8 = INITIAL_USE_COUNTER * 2;

#[derive(Debug)]
struct LfuMetadata {
    use_counter: u8,
    last_used_timestamp_in_mins: u16,
}

impl LfuMetadata {
    pub fn new() -> LfuMetadata {
        LfuMetadata {
            use_counter: INITIAL_USE_COUNTER,
            last_used_timestamp_in_mins: 0,
        }
    }
}

pub struct LfuEvictionPolicy<K: Eq + Hash, RNG: rand::Rng> {
    metadata: HashMap<K, LfuMetadata>,
    rng: RNG,
    start_timestamp_in_mins: i64,
    max_memory_in_bytes: u64,
    sample_size: u32,
    decay_time_in_mins: u16
}

impl<K: Eq + Hash, RNG: rand::Rng> LfuEvictionPolicy<K, RNG> {
    pub fn new(rng: RNG, start_timestamp: i64, max_memory_in_bytes: u64,
               sample_size: u32, decay_time_in_mins: u16)
               -> LfuEvictionPolicy<K, RNG>
    {
        LfuEvictionPolicy {
            metadata: HashMap::new(),
            rng: rng,
            start_timestamp_in_mins: start_timestamp / 60,
            max_memory_in_bytes: max_memory_in_bytes,
            sample_size: sample_size,
            decay_time_in_mins: decay_time_in_mins,
        }
    }

    fn update_metadata_for_key(&mut self, key: K) {
        let r = self.rng.next_f64();
        let start_timestamp_in_mins = self.start_timestamp_in_mins;

        let meta = self.metadata.entry(key).or_insert(LfuMetadata::new());
        meta.use_counter = increment_use_counter(meta.use_counter, r);
        meta.last_used_timestamp_in_mins = time_relative_to_start(
            start_timestamp_in_mins);
    }

    fn decrement_use_counter_if_necessary(&self, meta: &mut LfuMetadata) {
        let last_access = mins_since_last_access(
            meta, self.start_timestamp_in_mins);
        if last_access > self.decay_time_in_mins {
            meta.use_counter = decrement_use_counter(meta.use_counter);
        }
    }

}

impl<K: Eq + Hash, V, RNG: rand::Rng> EvictionPolicy<K, V>
    for LfuEvictionPolicy<K, RNG> {

    // Metadata management
    fn update_metadata_on_get(&mut self, key: K) {
        self.update_metadata_for_key(key);
    }

    fn update_metadata_on_set(&mut self, key: K, _: V) {
        self.update_metadata_for_key(key);
    }

    fn delete_metadata(&mut self, keys: &Vec<K>) {
        for key in keys {
            self.metadata.remove(key);
        }
    }

    fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Eviction policy logic
    fn should_evict_keys(&self, store_stats: &StoreStats) -> bool {
        store_stats.total_memory_in_bytes > self.max_memory_in_bytes
    }

    fn choose_keys_to_evict(&self) -> Vec<K> {
        Vec::new()
    }

}

fn decrement_use_counter(use_counter: u8) -> u8 {
    if use_counter > FAST_COUNTER_DECREMENT_THRESHOLD {
        let halved_counter = use_counter / 2;
        if halved_counter < FAST_COUNTER_DECREMENT_THRESHOLD {
            FAST_COUNTER_DECREMENT_THRESHOLD
        } else {
            halved_counter
        }
    } else {
        use_counter - 1
    }
}

// Pass in randomnly generated value between [0,1], so these functions don't
// depend on mutable state (which would be the RNG).
fn increment_use_counter(use_counter: u8, rand_val: f64) -> u8 {
    if use_counter == 255 {
        use_counter
    } else {
        logorithmically_increment_frequency(use_counter, rand_val)
    }
}

fn logorithmically_increment_frequency(counter: u8, rand_val: f64) -> u8 {
    let base_val = (counter - INITIAL_USE_COUNTER) as f64;
    let abs_base_val = if base_val > 0.0 { base_val } else { 0.0 };
    let probability = 1.0 / (abs_base_val * LFU_LOG_FACTOR + 1.0);
    if rand_val < probability {
        counter + 1
    } else {
        counter
    }
}

fn mins_since_last_access(meta: &LfuMetadata, start_timestamp_in_mins: i64)
    -> u16
{
    // TODO: handle overflow
    let relative_now = time_relative_to_start(start_timestamp_in_mins);
    relative_now - meta.last_used_timestamp_in_mins
}

fn time_relative_to_start(start_timestamp_in_mins: i64) -> u16 {
    let now_in_mins = time::now_utc().to_timespec().sec / 60;
    (now_in_mins - start_timestamp_in_mins) as u16
}
