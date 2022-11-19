use super::*;
use wasmi::*;
// use
use std::sync::{Arc, Mutex};

pub type SharedMemory<T> = Arc<Mutex<WasmMem<T>>>;

pub struct WasmMem<T> {
	pub store: SharedStore<T>,
	pub memory: Memory,
}

impl<T> WasmMem<T> {
	pub fn new(store: &SharedStore<T>, linker: &mut Linker<T>) -> SharedMemory<T> {
		let mut store_locked = store.lock().unwrap();
		let memory =
			Memory::new(&mut *store_locked, MemoryType::new(2, Some(16)).unwrap()).unwrap();
		linker.define("env", "memory", memory).unwrap();

		Arc::new(Mutex::new(WasmMem {
			store: Arc::clone(store),
			memory,
		}))
	}

	// pub fn data<'a>(&mut self) -> &[u8] {
	// 	let store = self.store.lock().unwrap();
	// 	self.memory.data(&*store)
	// }
}
