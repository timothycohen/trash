use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref GLOBAL: Global = Global::new();
}

pub struct Global {
    verbose: Mutex<bool>,
}

impl Global {
    pub fn new() -> Self {
        Global {
            verbose: Mutex::new(false),
        }
    }
    pub fn verbose(&self) -> bool {
        *self.verbose.lock().unwrap()
    }
    pub fn set_verbose(&self, verbose: bool) {
        *self.verbose.lock().unwrap() = verbose
    }
}

impl Default for Global {
    fn default() -> Self {
        Self::new()
    }
}
