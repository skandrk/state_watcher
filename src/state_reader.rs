use std::sync::{
    Arc, RwLock, RwLockReadGuard,
    atomic::{AtomicBool, Ordering},
};

#[derive(Clone)]
pub struct StateReader<T> {
    pub(crate) inner: Arc<(RwLock<T>, AtomicBool)>,
}

impl<T> StateReader<T>
where
    T: Clone,
{
    #[inline]
    fn lock<'a>(&'a self) -> RwLockReadGuard<'a, T> {
        match self.inner.0.read() {
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

    pub fn with_state<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.lock();
        f(&*guard)
    }
}
