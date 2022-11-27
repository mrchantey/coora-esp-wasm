use crate::*;
use coora_engine::*;
use wasmi::*;

fn link(app: &mut WasmApp) {
	let mut store = app.store.lock().unwrap();
	app.linker
		.define(
			"Test",
			"add",
			Func::wrap(&mut *store, move |_: Caller<UserState>, a: u32, b: u32| {
				println!("a: {a}, b: {b}, c:{}", a + b);
			}),
		)
		.unwrap();
}


sweet! {


	it "works" {
		let wasm = include_wasm!("../../../", "test_memory");

		let mut app = WasmApp::new();
		link(&mut app);
		app.build_with_wasm(&wasm[..]);

		// let mut mem_test = MemoryTestInstance::new(&mut app);
		// mem_test.printHello();
	}
}
