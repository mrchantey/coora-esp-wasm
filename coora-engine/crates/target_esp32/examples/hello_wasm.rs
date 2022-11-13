use coora_engine::{build_empty, build_hello_world};
use coora_target_esp32::*;
// use esp_idf_sys as _;
// esp_idf_sys::link_patches();
fn main() {
	// let mut instance = build_hello_world().unwrap();
	let mut instance = build_empty().unwrap();

	let add = instance.get_export::<(u64, u64), u64>("add");
	let result = add.call(&mut instance.store, (1, 2)).unwrap();

	println!("result is {}", result);
	// let str = String::from("hello cool world!");
	utility::sleep_forever();
}
