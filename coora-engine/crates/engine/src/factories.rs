use crate::*;
use anyhow::Result;
// use wasmi::*;
// use coora_engine::*;
pub fn build_hello_world() -> Result<WasmApp> {
	let _buf = include_wasm!("../../", "hello_world");


	todo!()
}


pub fn build_hello_led() -> Result<WasmApp> {
	//
	todo!()
}
pub fn build_mem_test() -> Result<MemoryTestInstance> {
	let wasm = include_wasm!("../../", "test_memory");

	let mut console = StdConsole {}.as_shared();
	let mut app = WasmApp::new();
	// link(&mut app);
	app.add_plugin(&mut console)?;
	app.build_with_wasm(&wasm[..]);

	let mem_test = MemoryTestInstance::new(&mut app);
	Ok(mem_test)
}
