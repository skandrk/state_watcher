use std::sync::{
    Arc, RwLock, RwLockWriteGuard,
    atomic::{AtomicBool, Ordering},
};

#[derive(Clone)]
pub struct StateWriter<T> {
    pub(crate) inner: Arc<(RwLock<T>, AtomicBool)>,
}

impl<T> StateWriter<T>
where
    T: Clone,
{
    fn lock<'a>(&'a self) -> RwLockWriteGuard<'a, T> {
        match self.inner.0.write() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
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
