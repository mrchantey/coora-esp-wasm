pub use coora_bindings::*;
pub use coora_engine::*;

fn main() {}

// #[coora_plugin]
trait Foo {
	fn do_thing();
	fn do_other_thing(a: u8);
}
// struct MyFoo;

trait MyTrait {}

struct MyT<T>(std::sync::Arc<std::sync::Mutex<T>>);
impl<T> Plugin for MyT<T> where T: Foo {
		fn bind<StoreT>(&mut self, _builder: &mut WasmInstanceBuilder<StoreT>) {}

}



// trait Plugin {
// }

// impl Plugin for MyFoo {}

// impl Foo for MyFoo {
// 	fn do_thing() {}
// 	fn do_other_thing(a: u8) {}
// }

// impl<T> Plugin for T
// where
// 	T: Foo,
// {
// 	fn bind<StoreT>(&mut self, builder: &mut WasmInstanceBuilder<StoreT>) {
// 		// builder.linker.//,,, self.mutex.lock_thing
// 	}
// }
// impl Plugin for Arc<Mutex<MyFoo>> {
// 	fn bind<StoreT>(&mut self, builder: &mut WasmInstanceBuilder<StoreT>) {
// 		//
// 		// builder.linker.//,,, self.mutex.lock_thing
// 	}
// }