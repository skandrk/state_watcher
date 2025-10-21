use std::sync::{
    Arc, Mutex, MutexGuard,
    atomic::{AtomicBool, Ordering},
};

#[derive(Clone)]
pub struct StateReadWriter<T> {
    pub(crate) inner: Arc<(Mutex<T>, AtomicBool)>,
}

impl<T> StateReadWriter<T>
where
    T: Clone,
{
    fn lock<'a>(&'a self) -> MutexGuard<'a, T> {
        match self.inner.0.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        }
    }

    pub fn latest(&self) -> Option<T> {
        if self.inner.1.load(Ordering::Acquire) {
            let guard = self.lock();
            Some(guard.clone())
        } else {
            None
        }
    }

    pub fn latest_and_clear(&self) -> Option<T> {
        if self.inner.1.swap(false, Ordering::AcqRel) {
            let guard = self.lock();
            Some(guard.clone())
        } else {
            None
        }
    }

    pub fn update(&self, t: T) {
        let mut guard = self.lock();
        *guard = t;
        drop(guard);
        self.inner.1.store(true, Ordering::Release);
    }

    pub fn with_state<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.lock();
        f(&*guard)
    }

    pub fn with_state_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.lock();
        f(&mut *guard)
    }
}
