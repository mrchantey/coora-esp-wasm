use crate::{plugins::TerminalLeds, *};
use coora_engine::*;

sweet! {

	let mut leds = TerminalLeds::new(2).as_shared();
	let mut app = WasmApp::new();
	app
		.add_plugin(&mut StdImports)?
		.add_plugin(&mut leds)?
		.build()?;

		let mut _sketch = SketchInstance::new(&mut app);

		test "recycle" {
			let mut app = app.recycle();
			app
			.add_plugin(&mut StdImports)?
			.add_plugin(&mut leds)?
				.build()?;
	}

}
