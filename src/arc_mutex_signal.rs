use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use dioxus::core::{Runtime, RuntimeGuard, ScopeId, spawn};

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

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        let inner = self.inner.clone();
        let val = *inner.lock().unwrap();
        val
    }
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
