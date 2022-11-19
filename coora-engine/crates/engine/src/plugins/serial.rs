use crate as coora_engine;
use crate::*;

//TODO actual strings not pointers
#[coora_import]
pub trait Serial {
	fn println(&mut self, ptr: u32, len: u32);
}


pub struct StdSerial {
	// start: SystemTime,
}

impl StdSerial {
	pub fn new() -> StdSerial {
		StdSerial {
			// start: SystemTime::now(),
		}
	}
}

impl Serial for StdSerial {
	fn println(&mut self, ptr: u32, len: u32) {
		//TODO print
		println!("tried to print: {ptr}, {len}");
	}
}
