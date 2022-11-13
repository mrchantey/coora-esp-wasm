use crate::{terminal_leds::TerminalLeds, *};
use coora_engine::*;
use forky_core::*;
use std::cell::RefCell;
use sweet::*;
use wasmi::*;

sweet! {

	let leds = TerminalLeds::new(2).as_shared();
	let mut instance = SketchInstance::from_default(&leds);
	let mut wasm = include_wasm!("../../../","hello_led");
	test "millis" {
		let a = instance._millis();
		forky_core::time::sleep(1);
		let b = instance._millis();
		expect((b - a) as i32).to_be_at_least(1000)?;
	}


	test "leds"{
		instance.run();
	}

	test "size"{
		expect(wasm.len() < 1000).to_be_true()?;
	}

	test skip "print"{
		println!("{:?}",wasm);
		println!("\n{} bytes\n",wasm.len());
	}
}
