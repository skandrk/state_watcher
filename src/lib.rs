pub mod state_reader;
pub mod state_writer;

#[cfg(feature = "mutex")]
pub mod state_readwriter;

pub use state_reader::StateReader;
pub use state_writer::StateWriter;

#[cfg(feature = "mutex")]
pub use state_readwriter::StateReadWriter;

use std::sync::{Arc, RwLock, atomic::AtomicBool};

#[cfg(feature = "mutex")]
use std::sync::Mutex;

pub fn state_channel<T>() -> (StateWriter<T>, StateReader<T>)
where
    T: Default + Clone,
{
    let inner = Arc::new((RwLock::new(T::default()), AtomicBool::new(false)));
    (
        StateWriter {
            inner: inner.clone(),
        },
        StateReader { inner },
    )
}

#[cfg(feature = "mutex")]
pub fn state_readerwriter<T>() -> StateReadWriter<T>
where
    T: Default + Clone,
{
    let inner = Arc::new((Mutex::new(T::default()), AtomicBool::new(false)));
    StateReadWriter { inner: inner }
}
