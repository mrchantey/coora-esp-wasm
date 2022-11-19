use crate::*;
use crate as coora_engine;
use wasmi::{TypedFunc};
// type Store = [u8; 16];
type Store = u32;
pub type SketchBuilder = WasmInstanceBuilder<Store>;
pub struct SketchInstance {
	pub run: TypedFunc<(), ()>,
	pub _millis: TypedFunc<(), u64>,
	pub instance: WasmInstance<Store>,
}

#[coora_export]
pub trait Sketchy{
	fn run();
	// fn setup();
	//fix this name
	fn _millis()->u64;
}


impl SketchInstance {
	#[rustfmt::skip]
	pub fn default_wasm() -> &'static [u8] { 
		include_wasm!("../../../", "hello_led") 
	}
}