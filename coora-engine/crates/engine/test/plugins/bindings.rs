use coora_bindings::coora_plugin;
use coora_engine::{WasmEngine, WasmInstanceBuilder};
use sweet::*;


#[coora_plugin]
pub trait MathPlugin {
	fn add(a: i32, b: i32) -> i32;
	// fn scale(a: i32);
	fn foo();
}

use std::sync::{Arc, Mutex};

sweet! {
	it "works" {
		let mut engine = WasmEngine::new();
		let mut builder = WasmInstanceBuilder::new(&mut engine, 0);

		let a = Arc::new(Mutex::new(2));

		let a1 = Arc::clone(&a);
		type StoreT = u32;
		let def = MathPluginDef::<StoreT>::new();
		def.bind_foo(&mut builder, move |_|{
			let mut locked = a1.lock().unwrap();
			*locked = *locked + 1;
		})
		.bind_add(&mut builder, |_,_,_|{3});
	}
}
