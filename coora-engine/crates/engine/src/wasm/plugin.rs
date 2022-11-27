use crate::WasmApp;
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub trait Shared
where
	Self: Sized,
{
	fn as_shared(self) -> Arc<Mutex<Self>> { Arc::new(Mutex::new(self)) }
}


pub trait Plugin
where
	Self: Sized,
{
	fn bind(&mut self, app: &mut WasmApp) -> Result<()>;
}
