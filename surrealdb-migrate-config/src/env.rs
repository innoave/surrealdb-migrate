//! A fakeable implementation of the `std::env` module.
//!
//! Inspired by the
//! [Testing code that uses environment variables](https://www.reddit.com/r/rust/comments/1jd8sxg/testing_code_that_uses_environment_variables/)
//! post on Reddit.

#[cfg(not(test))]
pub use std::env::{var, vars};

#[cfg(test)]
pub use fake_env::{remove_var, set_var, var, vars};

#[cfg(test)]
mod fake_env {
    use fakeenv::{EnvStore, Vars};
    use std::cell::RefCell;
    use std::env::VarError;

    thread_local! {
        static ENV_STORE: RefCell<EnvStore> = RefCell::new(EnvStore::fake());
    }

    pub fn var(name: &str) -> Result<String, VarError> {
        ENV_STORE.with(|env| env.borrow().var(name))
    }

    pub fn set_var(name: &str, value: &str) {
        ENV_STORE.with(|env| env.borrow_mut().set_var(name, value));
    }

    pub fn remove_var(name: &str) {
        ENV_STORE.with(|env| env.borrow_mut().remove_var(name));
    }

    pub fn vars() -> Vars {
        ENV_STORE.with(|env| env.borrow().vars())
    }
}
