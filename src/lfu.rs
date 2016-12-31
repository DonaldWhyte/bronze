extern crate rand;
extern crate time;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use self::rand::Rng;
use store::EvictionPolicy;
use store::StoreStats;

const LFU_LOG_FACTOR: f64 = 10.0;
const INITIAL_USE_FREQUENCY: u8 = 5;

struct LfuMetadata {
    use_frequency: u8,
    last_used_timestamp_in_mins: u16,
}

impl LfuMetadata {
    pub fn new() -> LfuMetadata {
        LfuMetadata {
            use_frequency: INITIAL_USE_FREQUENCY,
            last_used_timestamp_in_mins: 0,
        }
    }
}

pub struct LfuEvictionPolicy<K: Eq + Hash> {
    metadata: HashMap<K, LfuMetadata>,
    rng: rand::ThreadRng,
    start_timestamp_in_mins: i64,
}

impl<K: Eq + Hash> LfuEvictionPolicy<K> {
    pub fn new(start_timestamp: i64) -> LfuEvictionPolicy<K> {
        LfuEvictionPolicy {
            metadata: HashMap::new(),
            rng: rand::thread_rng(),
            start_timestamp_in_mins: start_timestamp / 60,
        }
    }

    fn update_metadata_for_key(&mut self, key: K) {
        let r = self.rng.next_f64();
        let start_timestamp_in_mins = self.start_timestamp_in_mins;

        let mut meta = self.metadata.entry(key).or_insert(LfuMetadata::new());
        meta.use_frequency = increment_use_frequency(meta.use_frequency, r);
        meta.last_used_timestamp_in_mins = time_relative_to_start(
            start_timestamp_in_mins);
    }

}

impl<K: Eq + Hash, V> EvictionPolicy<K, V> for LfuEvictionPolicy<K> {

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
        // TODO
        false
    }

    fn choose_keys_to_evict(&self) -> Vec<K> {
        // TODO
        Vec::new()
    }
}

// Pass in randomnly generated value between [0,1], so these functions don't
// depend on mutable state (which would be the RNG).
fn increment_use_frequency(use_frequency: u8, rand_val: f64) -> u8 {
    if use_frequency == 255 {
        use_frequency
    } else {
        logorithmically_increment_frequency(use_frequency, rand_val)
    }
}

fn logorithmically_increment_frequency(counter: u8, rand_val: f64) -> u8 {
    let base_val = (counter - INITIAL_USE_FREQUENCY) as f64;
    let abs_base_val = if base_val > 0.0 { base_val } else { 0.0 };
    let probability = 1.0 / (abs_base_val * LFU_LOG_FACTOR + 1.0);
    if rand_val < probability {
        counter + 1
    } else {
        counter
    }
}

fn time_relative_to_start(start_timestamp_in_mins: i64) -> u16 {
    let now_in_mins = time::now_utc().to_timespec().sec / 60;
    (now_in_mins - start_timestamp_in_mins) as u16
}
