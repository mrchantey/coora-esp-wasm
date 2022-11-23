use anyhow::Result;
use std::time::{Duration, SystemTime};

use crate::wifi;

pub struct Timeout {
    start: SystemTime,
    duration: Duration,
}

impl Timeout {
    pub fn new_wifi() -> Timeout {
        Self::new(wifi::TIMEOUT_DURATION)
    }
    pub fn new(duration: Duration) -> Timeout {
        Timeout {
            start: SystemTime::now(),
            duration,
        }
    }
    pub fn reset(&mut self) {
        self.start = SystemTime::now();
    }

    pub fn check_elapsed(&self) -> Result<bool> {
        let elapsed = SystemTime::now().duration_since(self.start)?;
        Ok(elapsed.ge(&self.duration))
    }
}
