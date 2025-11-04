dry run publish

# statewatcher

A simple, shared state channel where readers are notified of updates, inspired by `tokio::watch` but for `std`.

## Features

- **Zero dependencies** - Pure `std` implementation
- **Simple API** - Easy to understand and use
- **Thread-safe** - Built on `Arc` and `RwLock`/`Mutex`
- **Lightweight** - Minimal overhead using atomic flags for notifications
- **Optional mutex mode** - Use `Mutex` instead of `RwLock` for read-write access

## Why statewatcher?

A simple way to share state between threads with change notifications but without need to pull in Tokio or other async runtimes.

- Event-driven applications using `std::thread`
- Configuration updates across threads
- State synchronization in multi-threaded applications
- Simple pub-sub patterns without async complexity

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
statewatcher = "0.1.0"
```

### Basic Example

```rust
use statewatcher::state_channel;

fn main() {
    let (writer, reader) = state_channel::<String>();

    // Spawn a thread to watch for updates
    std::thread::spawn(move || {
        loop {
            if let Some(value) = reader.latest_and_clear() {
                println!("Received: {}", value);
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // Update the state from another thread
    writer.update("Hello, world!".to_string());
    std::thread::sleep(std::time::Duration::from_secs(1));
}
```

### Multiple Readers

```rust
use statewatcher::state_channel;

let (writer, reader1) = state_channel::<i32>();
let reader2 = reader1.clone();
let reader3 = reader1.clone();

writer.update(42);

// All readers see the update
assert_eq!(reader1.latest(), Some(42));
assert_eq!(reader2.latest(), Some(42));
assert_eq!(reader3.latest(), Some(42));
```

### Accessing State Without Cloning

```rust
use statewatcher::state_channel;

let (writer, reader) = state_channel::<Vec<i32>>();

writer.update(vec![1, 2, 3, 4, 5]);

// Access state without cloning
let sum = reader.with_state(|vec| vec.iter().sum::<i32>());
println!("Sum: {}", sum);
```

### Mutex Mode (Read-Write Handle)

Enable the `mutex` feature for a single handle that can both read and write:

```toml
[dependencies]
statewatcher = { version = "0.1.0", features = ["mutex"] }
```

```rust
use statewatcher::state_readerwriter;

fn main() {
    let state = state_readerwriter::<i32>();

    // Clone the handle for another thread
    let state_clone = state.clone();
    std::thread::spawn(move || {
        state_clone.update(42);
    });

    std::thread::sleep(std::time::Duration::from_millis(100));

    if let Some(value) = state.latest() {
        println!("Value: {}", value);
    }
}
```
