extern crate std;

use std::time::Instant;

pub struct Timer {
    start: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl Timer {
    pub fn elapsed(&self) -> u64 {
        (Instant::now() - self.start).as_millis() as u64
    }
}
