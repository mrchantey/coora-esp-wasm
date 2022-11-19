use crate::*;
use anyhow::Result;
// use wasmi::*;
pub fn build_hello_world() -> Result<WasmApp<u32>> {
	let _buf = include_wasm!("../../", "hello_world");


	todo!()
}


pub fn build_hello_led() -> Result<WasmApp<u32>> {
	//
	todo!()
}
