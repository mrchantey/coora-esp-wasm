use coora_bindings::coora_export;
pub use coora_engine::*;
fn main() {
	let a: FooInstance<u32>;

	// a.run();
	// a.ru
	// let a = FooInstance::new();
}


#[coora_export]
pub trait Foo {
	fn run();
	fn get_millis() -> u64;
	fn add(a: u32, b: u64) -> u32;
}
