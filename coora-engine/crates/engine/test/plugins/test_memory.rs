use crate::*;
use coora_engine::*;
use wasmi::*;


sweet! {


	it "works" {
		let wasm= 		include_wasm!("../../../", "test_memory");

		let mut serial = StdSerial::new().as_shared();

		let mut app = WasmApp::new(0);
		app.add_plugin(&mut serial).unwrap();

		// let mem = wasmi::Memory::new(&mut *store, wasmi::MemoryType::new(2, Some(16)).unwrap()).unwrap();
		// app.linker.define("env","memory",mem).unwrap();

		let mem = app.get_memory();
		app.build_with_wasm(&wasm[..]);
		// app.add_plugin

		let mut mem_test = MemoryTestInstance::new(&mut app);
		mem_test.printHello();

		let store = app.store.lock().unwrap();
		let a = &mem.data(& *store)[0x1036];
		// let a = &mem.data(& *store)[6800..6815];
		println!("{:?}",a);
		// mem.da
		// let memory = app.module.unwrap().
		// .get_memory(&mut *store, "memory")
		// .ok_or(anyhow::format_err!("failed to find `memory` export"))?;
		// app.build_with_sketch()

		// assert_eq!(memory.data(&store)[0x1002], 6);
		// assert_eq!(memory.data_mut(&mut store)[0x1000], 1);
	}
}
