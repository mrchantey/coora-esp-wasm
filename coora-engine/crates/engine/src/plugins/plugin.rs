// use std::sync::{Arc, Mutex};

use crate::WasmInstanceBuilder;
// pub type SharedLeds = Arc<Mutex<dyn LedStrip + Send>>;


// pub trait Plugin<T> {
// 	// type Shared = Arc<Mutex<dyn T + Send>>;
// 	// fn as_shared(self) -> Shared { Arc::new(Mutex::new(self)) }
// }

pub trait Plugin {
	fn bind<StoreT>(&mut self, builder: &mut WasmInstanceBuilder<StoreT>);
}
// trait Plugin {
// 	fn bind<StoreT>(&mut self, builder: &mut WasmInstanceBuilder<StoreT>) {}
// }

// fn as_shared(self) -> SharedLeds { Arc::new(Mutex::new(self)) }
