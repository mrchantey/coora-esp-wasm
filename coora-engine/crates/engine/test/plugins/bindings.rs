use std::marker::PhantomData;

use coora_bindings::coora_plugin;
use coora_engine::{WasmEngine, WasmInstanceBuilder};
// use std::sync::{Arc, Mutex};
use sweet::*;
use wasmi::{Caller, Func};
// use wasmi::{Caller, Engine, Func};


#[coora_plugin]
pub trait MathPlugin {
	// fn add(a: i32, b: i32) -> i32;
	// fn scale(a: i32);
	fn foo();
}

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

use std::sync::{Arc, Mutex};

sweet! {
	it "works" {
		let mut engine = WasmEngine::new();
		let mut _a = WasmInstanceBuilder::new(&mut engine, 0);

		let mut a= Arc::new(Mutex::new(2));

		let a1 = Arc::clone(&a);
		let p = define_math_plugin(MathPluginDef{
			_marker:PhantomData::<u32>,
			foo:move |_|{
				let mut locked = a1.lock().unwrap();
				*locked = *locked + 1;
				// a1 = a1 + 1
			}
		});
		_a.linker
		.define("MathPlugin", "foo", Func::wrap(&mut _a.store,p.foo))
		.unwrap();




		// let a = MathPluginDef{
		// 	_marker:PhantomStruct<u32>,
		// 	foo:|_|{}
		// 	// _marker:Phantom
		// };
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
