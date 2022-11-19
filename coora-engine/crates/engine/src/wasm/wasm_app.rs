use crate::{include_wasm, Plugin};
use anyhow::Result;
use std::sync::{Arc, Mutex};
use wasmi::*;
pub type SharedStore<T> = Arc<Mutex<Store<T>>>;

pub struct WasmApp<StoreT> {
	pub store: SharedStore<StoreT>,
	pub linker: Linker<StoreT>,
	pub instance: Option<Instance>,
	pub module: Option<Module>,
	pub engine: Engine,
}

impl<T> WasmApp<T> {
	pub fn recycle(self, initial_state: T) -> WasmApp<T> {
		WasmApp::new_with_engine(self.engine, initial_state)
	}

	pub fn new(initial_state: T) -> WasmApp<T> {
		Self::new_with_engine(WasmApp::<T>::default_engine(), initial_state)
	}
	pub fn new_with_engine(engine: Engine, initial_state: T) -> WasmApp<T> {
		let store = Store::new(&engine, initial_state);
		let linker = <Linker<T>>::new();
		WasmApp {
			engine,
			store: Arc::new(Mutex::new(store)),
			linker,
			instance: None,
			module: None,
		}
	}

	pub fn add_plugin<PluginT>(&mut self, plugin: &mut PluginT) -> Result<&mut Self>
	where
		PluginT: Plugin,
	{
		plugin.bind(self)?;
		Ok(self)
	}

	pub fn get_memory(&mut self) -> Memory {
		let mut store = self.store.lock().unwrap();
		let mem =
			wasmi::Memory::new(&mut *store, wasmi::MemoryType::new(2, Some(16)).unwrap()).unwrap();
		self.linker.define("", "", mem).unwrap();
		// self.linker.define("env", "memory", mem).unwrap();
		mem
	}

	pub fn build(&mut self) -> &mut Self { self.build_with_wasm(WasmApp::<T>::default_wasm()) }

	pub fn build_with_wasm(&mut self, stream: impl Read) -> &mut Self {
		let module = Module::new(&self.engine, stream).unwrap();
		let store = Arc::clone(&self.store);
		let mut store = store.lock().unwrap();
		let instance = self
			.linker
			.instantiate(&mut *store, &module)
			.unwrap()
			.start(&mut *store)
			.unwrap();
		self.instance = Some(instance);
		self.module = Some(module);
		self
	}
	pub fn default_wasm() -> &'static [u8] { include_wasm!("../../../", "hello_led") }

	pub fn default_engine() -> Engine {
		//https://github.com/barafael/wasm-on-mcu/blob/5303133d1c8b96d64452675ee486b05f26dc6e03/src/bin/wasmi.rs#L43
		//https://github.com/rustwasm/wasm-pack/issues/479
		//IMPORTANT - also set stack size compiler flag in .cargo/config.toml
		let config = Config::default();
		// config
		// config.set_stack_limits(StackLimits::new(256, 512, 128).unwrap());
		// config.wasm_features().bulk_memory = true;
		Engine::new(&config)
	}
}
