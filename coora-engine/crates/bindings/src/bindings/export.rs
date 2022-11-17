use crate::CooraPluginBindings;
use anyhow::Result;
use convert_case::{Case, Casing};
use std::{fs, path::PathBuf};



pub fn export_bindings() -> Result<()> {
	let out = PathBuf::from("target/bindings");
	let out_ts = out.join("typescript");
	let out_rs = out.join("rust");
	for path in [&out_ts, &out_rs] {
		if !path.exists() {
			fs::create_dir_all(path)?;
		}
	}

	let mut plugins = vec![];
	for plugin in inventory::iter::<CooraPluginBindings> {
		let plugin: &CooraPluginBindings = plugin;
		plugins.push(plugin);
		let camel_name = plugin.name.to_case(Case::Camel);

		fs::write(
			out_ts.join(plugin.name).with_extension("ts"),
			plugin.typescript_bindings,
		)?;
		fs::write(
			out_rs.join(camel_name).with_extension("rs"),
			plugin.rust_bindings,
		)?;
	}

	let mod_str: Vec<String> = plugins
		.iter()
		.map(|p| format!("pub mod {};", p.name.to_case(Case::Camel)))
		.collect();
	let mod_str = mod_str.join("\n");
	fs::write(out_rs.join("mod.rs"), mod_str)?;

	let index_str: Vec<String> = plugins
		.iter()
		.map(|p| format!("export * from \"./{}\";", p.name))
		.collect();
	let index_str = index_str.join("\n");
	fs::write(out_ts.join("index.ts"), index_str)?;


	Ok(())
}
