use std::marker::PhantomData;

pub use coora_bindings::*;

#[coora_plugin]
pub trait MyTrait {
	fn add() -> i32;
	fn mul(a: u8) -> i32;
	fn do_thing(a: i32, b: u8);
}

fn main() {
	let b = define_my_trait(MyTraitDef {
		_marker: PhantomData::<u64>,
		add: |_| 3,
		mul: |_, _| 3,
		do_thing: |_, _, _| {},
	});
	let b = MyTraitDef {
		_marker: PhantomData::<u32>,
		add: |_| 3,
		mul: |_, a| 3,
		do_thing: |_, _, _| {},
	};
}
