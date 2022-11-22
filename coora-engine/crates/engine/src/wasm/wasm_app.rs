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
	// pub module: Option<Module>,
	pub engine: Engine,
	pub memory: SharedMemory,
}

const INITIAL_PAGES: u32 = 1;
const MAX_PAGES: u32 = 1;

impl WasmApp {
	pub fn recycle(self) -> WasmApp { self.recycle_with(UserState::default()) }
	pub fn recycle_with(self, initial_state: UserState) -> WasmApp {
		WasmApp::new_with(self.engine, initial_state)
	}

	pub fn new() -> WasmApp { Self::new_with(WasmApp::default_engine(), UserState::default()) }

	pub fn create_memory(store: &SharedStore) -> SharedMemory {
		let mut store_locked = store.lock().unwrap();
		let memory = Memory::new(
			&mut *store_locked,
			MemoryType::new(INITIAL_PAGES, Some(MAX_PAGES)).unwrap(),
		)
		.unwrap();
		Arc::new(Mutex::new(memory))
	}


	pub fn new_with(engine: Engine, initial_state: UserState) -> WasmApp {
		let store = Store::new(&engine, initial_state);
		let linker = <Linker<UserState>>::new();
		let store = Arc::new(Mutex::new(store));
		let memory = Self::create_memory(&store);

		WasmApp {
			engine,
			store,
			linker,
			memory,
			instance: None,
			// module: None,
		}
	}
#[rustfmt::skip]
	pub fn link_memory(&mut self) -> Result<&mut Self> {
		let memory = *self.memory.lock().unwrap();
		self.linker.define("env", "memory", memory)?;
		Ok(self)
	}

	pub fn add_plugin<PluginT>(&mut self, plugin: &mut PluginT) -> Result<&mut Self>
	where
		PluginT: Plugin,
	{
		plugin.bind(self)?;
		Ok(self)
	}

	pub fn build(&mut self) -> Result<&mut Self> { self.build_with_wasm(Self::default_wasm()) }

	pub fn build_with_wasm(&mut self, stream: impl Read) -> Result<&mut Self> {
		self.link_memory()?; //would duplicate if we attempted reuse
		let module = Module::new(&self.engine, stream)?;
		let store = Arc::clone(&self.store);
		let mut store = store.lock().unwrap();
		let instance = self
			.linker
			.instantiate(&mut *store, &module)?
			.start(&mut *store)?;
		self.instance = Some(instance);
		// self.module = Some(module);
		Ok(self)
	}
	pub fn default_wasm() -> &'static [u8] { include_wasm!("../../../", "hello_led") }
	pub fn wasm_hello_world() -> &'static [u8] { include_wasm!("../../../", "hello_world") }

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
