use crate::{plugins::TerminalLeds, *};
use coora_engine::*;

sweet! {

	let mut leds = TerminalLeds::new(2).as_shared();
	let mut time = StdTime::new().as_shared();
	let mut engine = WasmEngine::new();

	let mut app = WasmInstanceBuilder::new(&mut engine, 0);
	#[rustfmt::skip]
	app
		.bind(&mut leds)
		.bind(&mut time);
		// .build(&mut engine, SketchInstance::default_wasm());
	let mut app = SketchInstance::build_with_default_sketch(&mut engine,app);

	test "millis" {
		let a = app._millis();
		forky_core::utility::sleep(1);
		let b = app._millis();
		expect((b - a) as i32).to_be_at_least(1000)?;
	}


	test "leds"{
		app.run();
	}

}
