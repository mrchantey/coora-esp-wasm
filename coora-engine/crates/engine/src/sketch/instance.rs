use crate::*;
use wasmi::{Read, TypedFunc};

// type Store = [u8; 16];
type Store = u32;
pub type SketchBuilder = WasmInstanceBuilder<Store>;

pub struct SketchInstance {
	pub run: TypedFunc<(), ()>,
	pub _millis: TypedFunc<(), u64>,
	pub instance: WasmInstance<Store>,
}


impl SketchInstance {
	pub fn build_with_default_sketch(
		engine: &mut WasmEngine,
		builder: SketchBuilder,
	) -> SketchInstance {
		let stream = include_wasm!("../../../", "hello_led");
		Self::build(engine, builder, &stream[..])
	}

	pub fn build(
		engine: &mut WasmEngine,
		builder: SketchBuilder,
		stream: impl Read,
	) -> SketchInstance {
		let mut instance = builder.build(engine, stream);
		let run = instance.get_export::<(), ()>("run");
		let _millis = instance.get_export::<(), u64>("_millis");
		SketchInstance {
			run,
			_millis,
			instance,
		}
	}

	pub fn run(&mut self) { self.run.call(&mut self.instance.store, ()).unwrap(); }
	pub fn _millis(&mut self) -> u64 { self._millis.call(&mut self.instance.store, ()).unwrap() }
}
