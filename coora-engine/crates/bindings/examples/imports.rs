pub use coora_engine::*;

fn main() {
	let mut a = MyFoo;
	a.do_thing();
	let b = a.as_shared();
	bind_the_things(b);
}


fn bind_the_things<T>(_plugin: T)
where
	T: Plugin,
{
	// plugin.bind(builder)
}
struct MyFoo;
// impl Shared for MyFoo {}
impl Foo for MyFoo {
	fn do_thing(&mut self) {}
	fn log(&self,_ptr: u32, _len: u32) {
		// self.st
	}
	fn do_other_thing(&self, _a: u32) {}
	fn add(&self, a: u32) -> u32 { a }
}

#[coora_import]
trait Foo {
	fn do_thing(&mut self);
	fn log(&self,ptr: u32, len: u32);
	fn do_other_thing(&self, a: u32);
	fn add(&self, a: u32) -> u32;
}
