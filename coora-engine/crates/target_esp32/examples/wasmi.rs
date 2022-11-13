// use anyhow::{anyhow, Result};
use coora_target_esp32::*;
use wasmi::*;
fn main() {
	let engine = Engine::default();
	let wasm = [
		0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x60,
		0x01, 0x7f, 0x00, 0x60, 0x00, 0x00, 0x02, 0x0e, 0x01, 0x04, 0x68, 0x6f,
		0x73, 0x74, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x00, 0x00, 0x03, 0x02,
		0x01, 0x01, 0x07, 0x09, 0x01, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x00,
		0x01, 0x0a, 0x08, 0x01, 0x06, 0x00, 0x41, 0x03, 0x10, 0x00, 0x0b,
	];

	let module = Module::new(&engine, &mut &wasm[..]).unwrap();
	type HostState = u32;
	let mut store = Store::new(&engine, 42);
	let host_hello =
		Func::wrap(&mut store, |caller: Caller<'_, HostState>, param: i32| {
			println!("Got {param} from WebAssembly");
			println!("My host state is: {}", caller.host_data());
		});
	let mut linker = <Linker<HostState>>::new();
	linker.define("host", "hello", host_hello).unwrap();
	let instance = linker
		.instantiate(&mut store, &module)
		.unwrap()
		.start(&mut store)
		.unwrap();
	let hello = instance
		.get_export(&store, "hello")
		.and_then(Extern::into_func)
		.ok_or_else(|| panic!("could not find function \"hello\""))
		.unwrap()
		.typed::<(), ()>(&mut store)
		.unwrap();

	hello.call(&mut store, ()).unwrap();
	// let add = instance
	// 	.get_export(&store, "add")
	// 	.and_then(Extern::into_func)
	// 	.ok_or_else(|| panic!("could not find function \"add\""))
	// 	.unwrap()
	// 	.typed::<(u32, u32), u32>(&mut store)
	// 	.unwrap();

	// let result = add.call(&mut store, (3, 2)).unwrap();
	// println!("the answer is {}", result);

	utility::sleep_forever();
}
