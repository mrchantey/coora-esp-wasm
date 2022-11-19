use crate::Plugin;
use std::sync::{Arc, Mutex};
use wasmi::*;
use anyhow::Result;
pub type SharedStore<T> = Arc<Mutex<Store<T>>>;

pub struct WasmApp<StoreT> {
	pub store: SharedStore<StoreT>,
	pub linker: Linker<StoreT>,
	pub instance: Option<Instance>,
}

impl<T> WasmApp<T> {

	pub fn new(engine: &mut Engine, initial_state: T) -> WasmApp<T> {
		let store = Store::new(&engine, initial_state);
		// store.state
		let linker = <Linker<T>>::new();
		WasmApp {
			store: Arc::new(Mutex::new(store)),
			linker,
			instance: None,
		}
	}

	pub fn add_plugin<PluginT>(&mut self, plugin: &mut PluginT) -> Result<&mut Self>
	where
		PluginT: Plugin,
	{
		plugin.bind(self)?;
		Ok(self)
	}

	pub fn build(&mut self, engine: &mut Engine, stream: impl Read) -> &mut Self {
		let module = Module::new(&engine, stream).unwrap();
		let store = Arc::clone(&self.store);
		let mut store = store.lock().unwrap();
		let instance = self
			.linker
			.instantiate(&mut *store, &module)
			.unwrap()
			.start(&mut *store)
			.unwrap();
		self.instance = Some(instance);
		self
	}
}
