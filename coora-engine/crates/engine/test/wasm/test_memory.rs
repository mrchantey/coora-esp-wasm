use crate::*;
use coora_engine::*;


sweet! {


	it "works" {
		let wasm = include_wasm!("../../../", "test_memory");

		let mut console = StdConsole{}.as_shared();
		let mut app = WasmApp::new();
		// link(&mut app);
		app.add_plugin(&mut console)?;
		app.build_with_wasm(&wasm[..]);

		let mut mem_test = MemoryTestInstance::new(&mut app);
		mem_test.printHello();
		mem_test.printHello();
		mem_test.printHello();

		// let str = str::from_utf8(&a).unwrap();
		// expect(str).to_be("hello from wasm")?;
	}
}

// fn link(app: &mut WasmApp) {
// 	let mut store = app.store.lock().unwrap();
// 	let memory = Arc::clone(&app.memory);
// 	app.linker
// 		.define(
// 			"Serial",
// 			"println",
// 			Func::wrap(
// 				&mut *store,
// 				move |caller: Caller<coora_engine::UserState>, ptr: u32, len: u32| {
// 					let memory = memory.lock().unwrap();
// 					let ctx = caller.as_context();
// 					println!("printing ptr: {ptr}, len: {len}");
// 					let data = &memory.data(ctx)[ptr as usize..(ptr + len) as usize];
// 					let str = str::from_utf8(&data).unwrap();
// 					println!("printing: {str}");

// 					// self3.lock().unwrap().add(a)
// 				},
// 			),
// 		)
// 		.unwrap();
// }
