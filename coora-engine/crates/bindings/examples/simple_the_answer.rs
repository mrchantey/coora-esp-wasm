pub use coora_bindings::*;
pub use coora_engine::*;

fn main() {
	type StoreT = u32;
	let _a = FooDef::<StoreT>::new();

	// _a.bind_do_thing(builder, func);
}

#[coora_plugin]
trait Foo {
	fn do_thing();
	fn do_other_thing(a: u8);
}


trait Plugin {
	fn bind<StoreT>(&mut self, builder: &mut WasmInstanceBuilder<StoreT>) {}
}

struct MyFoo;

impl Foo for MyFoo {
	fn do_thing() {}
	fn do_other_thing(a: u8) {}
}
impl Plugin for Arc<Mutex<MyFoo> {
	fn bind<StoreT>(&mut self, builder: &mut WasmInstanceBuilder<StoreT>) {
		//
		builder.linker.//,,, self.mutex.lock_thing


	}
}

fn bind<T>(foo: T)
where
	T: Foo,
{
}
