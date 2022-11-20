use crate::{include_wasm, Plugin};
use anyhow::Result;
use std::sync::{Arc, Mutex};
use wasmi::*;
pub type SharedStore = Arc<Mutex<Store<UserState>>>;
pub type SharedMemory = Arc<Mutex<Memory>>;

use super::*;
pub struct WasmApp {
	pub store: SharedStore,
	pub linker: Linker<UserState>,
	pub instance: Option<Instance>,
	pub module: Option<Module>,
	pub engine: Engine,
	pub memory: SharedMemory,
}

impl WasmApp {
	pub fn recycle(self) -> WasmApp { self.recycle_with(UserState::default()) }
	pub fn recycle_with(self, initial_state: UserState) -> WasmApp {
		WasmApp::new_with(self.engine, initial_state)
	}

	pub fn new() -> WasmApp { Self::new_with(WasmApp::default_engine(), UserState::default()) }

	pub fn create_memory(store: &SharedStore, linker: &mut Linker<UserState>) -> SharedMemory {
		let mut store_locked = store.lock().unwrap();
		let memory =
			Memory::new(&mut *store_locked, MemoryType::new(2, Some(16)).unwrap()).unwrap();
		linker.define("env", "memory", memory).unwrap();
		Arc::new(Mutex::new(memory))
	}


	pub fn new_with(engine: Engine, initial_state: UserState) -> WasmApp {
		let store = Store::new(&engine, initial_state);
		let mut linker = <Linker<UserState>>::new();
		let store = Arc::new(Mutex::new(store));
		let memory = Self::create_memory(&store, &mut linker);

		WasmApp {
			engine,
			store,
			linker,
			memory,
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

	pub fn build(&mut self) -> &mut Self { self.build_with_wasm(WasmApp::default_wasm()) }

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
		// config.wasm_features().bulk_memory = true;

		// let mut config = Config::default();
		// config.set_stack_limits(StackLimits::new(256, 512, 128).unwrap());
		// Engine::new(&config)
		Engine::default()
	}
}
