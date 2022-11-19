// use std::error::Error;
use anyhow::{anyhow, Result};
use coora_bindings::coora_export;
pub use coora_engine::*;
fn main() {}
trait Foo {
	fn run();
	fn get_millis() -> u64;
	fn add(a: u32, b: u64) -> u32;
}
struct FooInstance<T> {
	store: coora_engine::SharedStore<T>,
	_run: wasmi::TypedFunc<(), ()>,
	_get_millis: wasmi::TypedFunc<(), u64>,
	_add: wasmi::TypedFunc<(u32, u64), u32>,
}
impl<T> FooInstance<T> {
	pub fn new(&mut self, app: &mut WasmApp<T>) -> FooInstance<T> {
			let instance = Some(app.instance).unwrap().unwrap();
			let store = std::sync::Arc::clone(&app.store);
			let mut store_locked = store.lock().unwrap();
			FooInstance::<T> {
					store: std::sync::Arc::clone(&app.store),
					_run: instance
							.get_export(&mut *store_locked, "run")
							.and_then(wasmi::Extern::into_func)
							.ok_or_else(|| panic!())
							.unwrap()
							.typed::<(), ()>(&mut *store_locked)
							.unwrap(),
							_get_millis: instance
							.get_export(&mut *store_locked, "get_millis")
							.and_then(wasmi::Extern::into_func)
							.ok_or_else(|| panic!())
							.unwrap()
							.typed::<(), u64>(&mut *store_locked)
							.unwrap(),
							_add: instance
							.get_export(&mut *store_locked, "add")
							.and_then(wasmi::Extern::into_func)
							.ok_or_else(|| panic!())
							.unwrap()
							.typed::<(u32, u64), u32>(&mut *store_locked)
							.unwrap(),
			}
	}
}
