use coora_bindings::*;
use coora_engine::*;
use sweet::*;


#[coora_import]
pub trait MathPlugin {
	fn add(&self, a: i32, b: i32) -> i32;
	// fn scale(a: i32);
	fn foo(&self);
}

#[coora_export]
pub trait MyExports{
	fn run();
}

sweet! {
	it "works" {
		let mut engine = WasmEngine::new();
		let mut app = WasmApp::new(&mut engine, 0);
	
		app.build(&mut engine,SketchInstance::default_wasm());

		// let plugin = MyMathImpl;

		// builder.

		// let a = Arc::new(Mutex::new(2));

		// let a1 = Arc::clone(&a);
		// type StoreT = u32;

	}
}
