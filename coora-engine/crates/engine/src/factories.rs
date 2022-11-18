use crate::*;
use anyhow::Result;
// use wasmi::*;
pub fn build_hello_world() -> Result<WasmInstance<u32>> {
	todo!()
	// let mut engine = WasmEngine::new();
	// let buf = include_rust_wasm!("../../", "hello_world");
	// let buf = include_wasm!("../../", "hello_world");
	// let mut builder = engine.instantiate(0);
	// builder.add_import_1("host", "howdy", |mut caller, param: i32| {
	// 	let data = caller.host_data_mut();
	// 	*data = *data + param as u32;
	// });
	// let instance = builder.build(&mut engine, &buf[..]);
	// Ok(instance)
}
// pub fn build_empty() -> Result<WasmInstance<u32>> {
// 	let mut engine = WasmEngine::new();
// 	let buf = include_rust_wasm!("../../");
// 	let builder = engine.instantiate(0);
// 	let instance = builder.build(&mut engine, &buf[..]);
// 	Ok(instance)
// }
