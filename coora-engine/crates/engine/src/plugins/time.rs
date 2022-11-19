use std::time::SystemTime;

use crate as coora_engine;
use crate::*;


#[coora_import]
pub trait Time {
	// fn as_shared(self) -> SharedLeds;
	fn elapsed(&mut self) -> u64;
}


pub struct StdTime {
	start: SystemTime,
}

impl StdTime {
	pub fn new() -> StdTime {
		StdTime {
			start: SystemTime::now(),
		}
	}
}

impl Time for StdTime {
	fn elapsed(&mut self) -> u64 {
		std::time::SystemTime::now()
			.duration_since(self.start)
			.unwrap()
			.as_millis() as u64
	}
}
