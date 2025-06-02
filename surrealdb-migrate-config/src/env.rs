//! inspired by
//! [Testing code that uses environment variables](https://www.reddit.com/r/rust/comments/1jd8sxg/testing_code_that_uses_environment_variables/)
//! - post on Reddit

#[cfg(not(test))]
pub use std::env::{var, vars};

#[cfg(test)]
pub use stubbed_vars::{remove_var, set_var, var, vars};

#[cfg(test)]
mod stubbed_vars {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::env::VarError;

    thread_local! {
        static ENV_VARS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    }

    pub fn var(name: &str) -> Result<String, VarError> {
        ENV_VARS
            .with(|env| env.borrow().get(name).cloned())
            .ok_or(VarError::NotPresent)
    }

    pub fn set_var(name: &str, value: &str) {
        ENV_VARS.with(|env| env.borrow_mut().insert(name.to_string(), value.to_string()));
    }

    pub fn remove_var(name: &str) {
        ENV_VARS.with(|env| env.borrow_mut().remove(name));
    }

    pub fn vars() -> Vars {
        ENV_VARS.with(|env| Vars {
            values: env
                .borrow()
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
            next: 0,
        })
    }

    pub struct Vars {
        values: Vec<(String, String)>,
        next: usize,
    }

    impl Iterator for Vars {
        type Item = (String, String);

        fn next(&mut self) -> Option<Self::Item> {
            let next = self.values.get(self.next).cloned();
            if next.is_some() {
                self.next += 1;
            }
            next
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.values.len() - self.next;
            (remaining, Some(remaining))
        }
    }

    impl ExactSizeIterator for Vars {}
}
