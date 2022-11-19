use coora_bindings::coora_export;
pub use coora_engine::*;
fn main() {
	let _a: FooInstance<u32>;
}


#[coora_export]
pub trait Foo {
	fn run();
	fn get_millis() -> u64;
	fn add(a: u32, b: u64) -> u32;
}
