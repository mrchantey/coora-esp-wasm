use crate::CooraPluginBindings;
use anyhow::Result;
use std::{
	fs,
	path::{Path, PathBuf},
};



pub fn export_bindings() -> Result<()> {
	let out = PathBuf::from("bindings");
	let out_ts = out.join("typescript");
	let out_rs = out.join("rust");
	for path in [&out, &out_ts, &out_rs] {
		if !path.exists() {
			fs::create_dir(path)?;
		}
	}

	for plugin in inventory::iter::<CooraPluginBindings> {
		let plugin: &CooraPluginBindings = plugin;

		fs::write(out_ts.join(plugin.name), plugin.typescript_bindings)?;
		fs::write(out_rs.join(plugin.name), plugin.rust_bindings)?;
	}

	Ok(())
}
