use crate::*;
use anyhow::Result;
// use wasmi::*;
pub fn build_hello_world() -> Result<WasmApp> {
	let _buf = include_wasm!("../../", "hello_world");


	todo!()
}


pub fn build_hello_led() -> Result<WasmApp> {
	//
	todo!()
}
