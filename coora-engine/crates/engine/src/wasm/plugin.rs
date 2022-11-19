use crate::WasmApp;
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub trait Shared
where
	Self: Sized,
{
	fn as_shared(self) -> Arc<Mutex<Self>> { Arc::new(Mutex::new(self)) }
}


pub trait Plugin {
	fn bind<StoreT>(&mut self, builder: &mut WasmApp<StoreT>) -> Result<()>;
}
