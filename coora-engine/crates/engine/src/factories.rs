use crate::*;
use anyhow::Result;
// use wasmi::*;
// use coora_engine::*;
pub fn build_hello_world() -> Result<WasmApp> {
	let wasm = include_wasm!("../../", "hello_world");
	let mut app = WasmApp::new();
	app.build_with_wasm(&wasm[..])?;
	let mut adder = AdderInstance::new(&mut app);
	let result = adder.add(1, 2);
	println!("{result}");

	todo!()
}

pub fn build_mem_test() -> Result<MemoryTestInstance> {
	let wasm = include_wasm!("../../", "test_memory");

	let mut console = StdDebug {}.as_shared();
	let mut app = WasmApp::new();
	// link(&mut app);
	app.add_plugin(&mut console)?;
	app.build_with_wasm(&wasm[..])?;

	let mem_test = MemoryTestInstance::new(&mut app);
	Ok(mem_test)
}
