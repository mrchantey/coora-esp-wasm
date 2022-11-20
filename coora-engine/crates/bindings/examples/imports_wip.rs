// pub use coora_engine::*;

fn main() {}


// // #[coora_import]
// trait Foo {
// 	fn println(&mut self, val: &str);
// }

// pub struct DeorphanedFoo<T>(std::sync::Arc<std::sync::Mutex<T>>);
// pub trait SharedFoo
// where
// 	Self: Sized,
// {
// 	fn as_shared(self) -> DeorphanedFoo<Self> {
// 		DeorphanedFoo::<Self>(std::sync::Arc::new(std::sync::Mutex::new(self)))
// 	}
// }
// impl<T> SharedFoo for T where T: Foo {}
// impl<T> coora_engine::Plugin for DeorphanedFoo<T>
// where
// 	T: Foo + std::marker::Send + 'static,
// {
// 	fn bind(&mut self, app: &mut coora_engine::WasmApp) -> anyhow::Result<()> {
// 		let store = std::sync::Arc::clone(&app.store);
// 		let mut store = store.lock().unwrap();
// 		let self0 = std::sync::Arc::clone(&self.0);
// 		let mem0 = Arc::clone(&app.memory);

// 		app.linker
// 			.define(
// 				"Foo",
// 				"println",
// 				wasmi::Func::wrap(
// 					&mut *store,
// 					move |_: wasmi::Caller<coora_engine::UserState>| {
// 						self0.lock().unwrap().println()
// 					},
// 				),
// 			)
// 			.unwrap();
// 		Ok(())
// 	}
// }
