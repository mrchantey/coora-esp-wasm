use crate as coora_engine;
use crate::*;
//TODO actual strings not pointers
#[coora_import]
pub trait Serial {
	fn println(&mut self, ptr: u32, len: u32);
}
pub struct SafeThing {}

pub struct StdSerial {
	// thing: Rc<RefCell<Store<UserState>>>, // memory: SharedMemory, // start: SystemTime,
	memory: SharedMemory,
}

impl StdSerial {
	pub fn new(memory: SharedMemory) -> StdSerial {
		StdSerial {
			memory, // start: SystemTime::now(),
		}
	}
}

impl Serial for StdSerial {
	fn println(&mut self, ptr: u32, len: u32) {
		// println!("here we are");
		let memory = &self.memory.lock().unwrap();
		// let store = memory.store.lock().unwrap();
		// let a = &memory.memory.data(&*store)[6800..6815];
		// //TODO print
		// println!("the text is {}", str::from_utf8(&a).unwrap());
	}
}
