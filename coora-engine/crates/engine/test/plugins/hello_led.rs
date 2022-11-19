use crate::{plugins::TerminalLeds, *};
use coora_engine::*;

sweet! {

	let mut leds = TerminalLeds::new(2).as_shared();
	let mut time = StdTime::new().as_shared();
	let mut engine = WasmEngine::new();
	let mut app = WasmApp::new(&mut engine, 0);
	app
		.add_plugin(&mut leds).unwrap()
		.add_plugin(&mut time).unwrap()
		.build(&mut engine);

	let mut sketch = SketchyInstance::new(&mut app);

	test "millis" {
		let a = sketch._millis();
		forky_core::utility::sleep(1);
		let b = sketch._millis();
		expect((b - a) as i32).to_be_at_least(1000)?;
	}


	test "leds"{
		sketch.run();
	}

}
