use crate::*;
use coora_engine::*;

sweet! {
	let wasm = include_wasm!("../../../","hello_led");

	test "size"{
		expect(wasm.len() < 1000).to_be_true()?;
	}

	test skip "print"{
		println!("{:?}",wasm);
		println!("\n{} bytes\n",wasm.len());
	}

}
