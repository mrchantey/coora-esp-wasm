use anyhow::Result;
pub use coora_bindings::*;

#[coora_plugin]
pub trait Math {
	fn add(a: i32, b: f32) -> i32;
	fn mul(a: i32, b: i32) -> i32;
	fn set_stuff(a: i32, b: String);
}
#[coora_plugin]
pub trait Core {
	fn do_thing_here();
	fn do_thing_there(a: usize);
}




pub fn main() -> Result<()> {
	export_bindings()?;
	Ok(())
}
