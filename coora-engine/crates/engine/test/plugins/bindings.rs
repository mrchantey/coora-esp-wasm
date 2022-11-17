use coora_bindings::coora_plugin;
use coora_engine::{WasmEngine, WasmInstanceBuilder};
use std::sync::{Arc, Mutex};
use sweet::*;
use wasmi::{Caller, Engine, Func};

// pub type SharedMathPlugin = Arc<Mutex<dyn MathPlugin + Send>>;

// #[coora_plugin]
// pub trait MathPlugin {
// 	fn add(a: i32, b: i32) -> i32;
// 	fn scale(a: i32);
// 	fn foo();
// }

// pub struct MyFuncImpl {
// 	instance: &SharedMathPlugin,
// }
// impl MyFuncImpl {
// 	fn new(instance: SharedMathPlugin) -> MyFuncImpl {
// 		//
// 		MyFuncImpl { instance }
// 	}
// 	fn do_add(builder: &mut SketchBuilder) {}
// }
// fn create_thing(){

// };


struct Foo<T, T2>
where
	T2: FnMut(Caller<T>) -> u32,
	T2: FnMut(Caller<T>),
{
	pub val: T,
	pub my_func: T2,
}


sweet! {
	it "works" {
		let mut engine = WasmEngine::new();
		let a = WasmInstanceBuilder::new(&mut engine, 0);
		// let a = Foo{
		// 	val:3,
		// 	my_func: move |c|{

		// 	}
		// };
		// let closure:dyn FnMut(Caller<u32>) = |a:Caller<u32>|{};
		// let foo = Func::wrap(a.store,closure);
		// expect(true).to_be(false)?;
		// Func::wrap()
	}
}
