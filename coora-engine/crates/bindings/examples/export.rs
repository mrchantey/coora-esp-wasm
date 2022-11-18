use anyhow::Result;
pub use coora_bindings::*;
pub use coora_engine::*;

// #[coora_plugin]
// pub trait Math {
// 	fn add(&self,a: i32, b: i64) -> i32;
// 	fn mul(&self,a: i32, b: u32) -> i32;
// 	fn set_stuff(&self,a: i32, b: u32);
// }

// #[coora_plugin]
// pub trait Core {
// 	fn do_thing_here(&mut self);
// 	fn do_thing_there(&self,a: i32);
// }




pub fn main() -> Result<()> {
	export_bindings()?;
	Ok(())
}
