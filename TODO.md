# statewatcher Publishing TODO

### Complete Documentation

- [ ] **Document all public items**
  - [ ] `src/lib.rs` - Functions and module docs
  - [ ] `src/state_reader.rs` - All public methods (use `outputs/documented_state_reader.rs`)
  - [ ] `src/state_writer.rs` - All public methods
  - [ ] `src/state_readwriter.rs` - All public methods

- [ ] **Add doc examples**
  - [ ] Each public method should have a `# Examples` section
  - [ ] Run `cargo test --doc` to verify examples compile

### Testing

- [ ] **Add test suite**
  - [ ] Copy tests from `outputs/tests.rs` to `src/lib.rs` or `tests/`
  - [ ] Add at least these tests:
    - [ ] Basic update and read
    - [ ] `latest_and_clear()` behavior
    - [ ] Multiple readers
    - [ ] Concurrent access
    - [ ] `with_state()` functionality
    - [ ] `with_state_mut()` behavior

### Examples

- [ ] **Create examples directory**

  ```bash
  mkdir examples
  ```

- [ ] **Add basic example**
  - [ ] Copy `outputs/basic_example.rs` to `examples/basic.rs`
  - [ ] Test: `cargo run --example basic`

- [ ] **Add multi-threaded example**
  - [ ] Copy `outputs/multi_threaded_example.rs` to `examples/multi_threaded.rs`
  - [ ] Test: `cargo run --example multi_threaded`

### Important Files

- [ ] **Add CHANGELOG.md**
  - [ ] Copy from `outputs/CHANGELOG.md`
  - [ ] Update date to actual publish date

---

### Code Improvements

- [ ] **Document `with_state_mut` caveat**
  - [ ] Add big warning in docs that it doesn't set notification flag
  - [ ] Consider adding `mark_updated()` method

- [ ] **Consider adding methods**
  - [ ] `StateReader::clear()` - Clear flag without reading
  - [ ] `StateReader::has_changed()` - Check without consuming
  - [ ] `StateWriter::mark_updated()` - Manually set flag after `with_state_mut`

- [ ] **Add Debug trait**
  - [ ] Implement Debug for StateReader (careful with inner state)
  - [ ] Implement Debug for StateWriter
  - [ ] Implement Debug for StateReadWriter

### Documentation Improvements

- [ ] **Add comparison section to README**
  - [ ] vs tokio::watch
  - [ ] vs Arc<RwLock<T>>
  - [ ] vs std::mpsc
  - [ ] vs crossbeam-channel

- [ ] **Add use cases section**
  - [ ] Configuration hot-reloading
  - [ ] State broadcasting
  - [ ] Event-driven architectures

- [ ] **Add migration guide**
  - [ ] From Arc<RwLock<T>>
  - [ ] From tokio::watch (for those going sync)

---

## 🎯 Nice to Have (Future Versions)

- [ ] **Add `mark_updated()` method**

- [ ] **Add `clear()` method for readers**

- [ ] **Make `with_state_mut` set flag**
  - [ ] Or add separate `with_state_mut_notify()`
  - [ ] Or return a guard that sets on drop

- [ ] **Add `has_changed()` method**

### Performance

- [ ] **Add benchmarks**
  - [ ] Create `benches/` directory
  - [ ] Compare vs Arc<RwLock<T>> directly
  - [ ] Compare different workloads (single reader, multi reader, etc.)

- [ ] **Profile memory usage**
  - [ ] Document overhead per reader/writer
  - [ ] Test with large state objects

### Additional Features

- [ ] **Consider async support**
  - [ ] Feature flag: `async`
  - [ ] `async fn changed()` method for readers
  - [ ] Requires tokio or async-std dependency

- [ ] **Consider no_std support**
  - [ ] Feature flag: `no_std`
  - [ ] Replace RwLock with spin::RwLock
  - [ ] Would need alloc for Arc

- [ ] **Add metrics/debugging**
  - [ ] Reader count
  - [ ] Update count
  - [ ] Flag state inspection

---
