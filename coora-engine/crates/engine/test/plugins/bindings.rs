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
pub trait MyExports {
	fn run();
}

sweet! {
	it "works" {
		let mut _app = WasmApp::new();

		// app.build();


	}
}
