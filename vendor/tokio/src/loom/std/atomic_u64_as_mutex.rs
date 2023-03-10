use crate::loom::sync::Mutex;
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub(crate) struct AtomicU64 {
    inner: Mutex<u64>,
}

impl AtomicU64 {
    pub(crate) fn new(val: u64) -> Self {
        Self {
            inner: Mutex::new(val),
        }
    }

    pub(crate) fn load(&self, _: Ordering) -> u64 {
        *self.inner.lock()
    }

    pub(crate) fn store(&self, val: u64, _: Ordering) {
        *self.inner.lock() = val;
    }

    pub(crate) fn fetch_add(&self, val: u64, _: Ordering) -> u64 {
        let mut lock = self.inner.lock();
        let prev = *lock;
        *lock = prev + val;
        prev
    }

    pub(crate) fn fetch_or(&self, val: u64, _: Ordering) -> u64 {
        let mut lock = self.inner.lock();
        let prev = *lock;
        *lock = prev | val;
        prev
    }

    pub(crate) fn compare_exchange(
        &self,
        current: u64,
        new: u64,
        _success: Ordering,
        _failure: Ordering,
    ) -> Result<u64, u64> {
        let mut lock = self.inner.lock();

        if *lock == current {
            *lock = new;
            Ok(current)
        } else {
            Err(*lock)
        }
    }

    pub(crate) fn compare_exchange_weak(
        &self,
        current: u64,
        new: u64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u64, u64> {
        self.compare_exchange(current, new, success, failure)
    }
}

impl Default for AtomicU64 {
    fn default() -> AtomicU64 {
        AtomicU64::new(u64::default())
    }
}
