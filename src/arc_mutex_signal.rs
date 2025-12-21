use std::sync::{Arc, Mutex};

/// implements a type that stores value within an arc-mutex and
/// calls dioxus::core::needs_update when updating the value.

pub struct AMSignal<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> AMSignal<T> {
    pub fn new(val: T) -> AMSignal<T> {
        AMSignal {
            inner: Arc::new(Mutex::new(val)),
        }
    }

    /// If T doesn't implemet Copy trait, consider using get_inner()  
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        let inner = self.inner.clone();
        let val = *inner.lock().unwrap();
        val
    }
    /// Returns a clone of inner arc mutex
    // pub fn get_inner(&self) -> Arc<Mutex<T>>
    // where
    //     T: Clone,
    // {
    //     self.inner.clone()
    // }

    pub fn set(&self, new: T) {
        let inner = self.inner.clone();
        *inner.lock().unwrap() = new;

        dioxus::core::needs_update();
    }
    pub fn clone(&self) -> AMSignal<T> {
        AMSignal {
            inner: self.inner.clone(),
        }
    }
}
