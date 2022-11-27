//
use super::*;
use crate::{Plugin, WasmApp};


pub struct StdImports;


impl Plugin for StdImports {
	fn bind(&mut self, app: &mut WasmApp) -> anyhow::Result<()> {
		let mut time = StdTime::new().as_shared();
		let mut math = StdMath::new().as_shared();
		let mut debug = StdDebug::default().as_shared();


		app.add_plugin(&mut math)?
			.add_plugin(&mut time)?
			.add_plugin(&mut debug)?;
		Ok(())
	}
}
