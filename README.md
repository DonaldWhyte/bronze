# bronze

Single-threaded in-memory cache implemented in Rust.

### TODO List

- [ ] Add rustfmt as dependency and use in Makefile
- [ ] Decide on eviction policy will be represented
        - make eviction logic and metadata retrieval same trait?
        - `should_evict` and `choose_keys_to_evict`?
- [ ] Implement LFU eviction policy
- [ ] Come up framework/scripts to test efficiency of eviction policies
- [ ] Document everything
- [ ] Write unit tests for everything