use crate::*;
use coora_engine::*;
use std::str;

sweet! {


	it "works" {
		let wasm = include_wasm!("../../../", "test_memory");
		let mut serial = StdSerial::new().as_shared();

		let mut app = WasmApp::new(0);
		app.add_plugin(&mut serial).unwrap();
		app.build_with_wasm(&wasm[..]);

		let mut mem_test = MemoryTestInstance::new(&mut app);
		mem_test.printHello();

		let store = app.store.lock().unwrap();
		let a = &app.memory.lock().unwrap().memory.data(&*store)[6800..6815];
		let str = str::from_utf8(&a).unwrap();
		expect(str).to_be("hello from wasm")?;
	}
}
