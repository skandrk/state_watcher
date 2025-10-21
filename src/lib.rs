pub mod state_reader;
pub mod state_readwriter;
pub mod state_writer;

pub use state_reader::StateReader;
pub use state_writer::StateWriter;
use std::sync::{Arc, Mutex, RwLock, atomic::AtomicBool};

use crate::state_readwriter::StateReadWriter;

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

pub fn state_readerwriter<T>() -> StateReadWriter<T>
where
    T: Default + Clone,
{
    let inner = Arc::new((Mutex::new(T::default()), AtomicBool::new(false)));
    StateReadWriter { inner: inner }
}
