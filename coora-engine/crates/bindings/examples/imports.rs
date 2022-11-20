pub use coora_engine::*;

fn main() {}



#[coora_import]
trait Foo {
	fn add(&mut self, a: u32,b:u32)->u32;
	// fn println(&mut self, val: &str);
}
