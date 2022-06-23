use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref GLOBAL: Global = Global::new();
}

pub struct Global {
    verbose: Mutex<bool>,
    force: Mutex<bool>,
}

impl Global {
    pub fn new() -> Self {
        Global {
            verbose: Mutex::new(false),
            force: Mutex::new(false),
        }
    }
    pub fn verbose(&self) -> bool {
        *self.verbose.lock().unwrap()
    }
    pub fn set_verbose(&self, verbose: bool) {
        *self.verbose.lock().unwrap() = verbose
    }
    pub fn force(&self) -> bool {
        *self.force.lock().unwrap()
    }
    pub fn set_force(&self, force: bool) {
        *self.force.lock().unwrap() = force
    }
}

impl Default for Global {
    fn default() -> Self {
        Self::new()
    }
}
