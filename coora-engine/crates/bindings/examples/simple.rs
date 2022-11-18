pub use coora_engine::*;

fn main() {}

#[coora_plugin]
trait Foo {
	fn do_thing(&mut self);
	fn do_other_thing(&self, a: u32);
	fn add(&self, a: u32) -> u32;
}

#[coora_plugin]
pub trait Core {
	// fn do_other(&mut self);
	fn do_thing_here(&mut self);
	fn do_thing_there(&self, a: i32);
}

fn bind_the_things<T>(plugin: T)
where
	T: Plugin,
{
	// plugin.bind(builder)
}
struct MyFoo;

impl Foo for MyFoo {
	fn do_thing(&mut self) {}
	fn do_other_thing(&self, a: u32) {}
	fn add(&self, a: u32) -> u32 { 2 }
}
