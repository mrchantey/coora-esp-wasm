pub use coora_bindings::*;


#[coora_plugin]
pub trait MyTrait {
	fn add() -> i32;
	fn mul() -> i32;
	fn do_thing(a: i32, b: String);
}



// fn main() {}
fn main() {}
