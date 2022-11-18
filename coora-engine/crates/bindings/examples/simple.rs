pub use coora_bindings::*;
pub use coora_engine::*;

fn main() {
	type StoreT = u32;
	let _a = FooDef::<StoreT>::new();

}

#[coora_plugin]
trait Foo {
	fn do_thing();
	fn do_other_thing(a:u8);
}
