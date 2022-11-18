pub use coora_bindings::*;
pub use coora_engine::*;
use std::marker::PhantomData;
use wasmi::*;

trait Foo {
	fn do_thing();
}
// impl Foo{

// }

struct FooDef;

impl FooDef{
	fn bind_do_thing(){
			
	}
}


fn main() {
	// Foo::do_thing();
	// let _ = build_my_trait()
	// let _ = define_my_trait(MyTraitDef {
	// 	_marker: PhantomData::<u64>,
	// 	add: |_| 3,
	// mul: |_, _| 3,
	// do_thing: |_, _, _| {},
	// });
	// let _ = MyTraitDef {
	// 	_marker: PhantomData::<u32>,
	// 	add: |_| 3,
	// add_inputs: (wasmi::Caller::(9), 2),
	// add_inputs: (PhantomData::<wasmi::Caller<'_, u32>>,PhantomData::<u32>),
	// mul: |_, _| 3,
	// do_thing: |_, _, _| {},
	// };
}

// #[coora_plugin]
// pub trait MyTrait {
// 	fn add() -> i32;
// 	// fn mul(a: u8) -> i32;
// 	// fn do_thing(a: i32, b: u8);
// }
