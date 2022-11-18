use crate::CooraPluginBindings;
use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::{fs, path::PathBuf};

//TODO decouple rust and ts

const EXPORT_ROOT: &str = "target/bindings";

pub fn export_and_collect<T>(out_dir: &PathBuf, write_file: T) -> Result<Vec<&CooraPluginBindings>>
where
	T: Fn(&CooraPluginBindings) -> (PathBuf, &str),
{
	fs::remove_dir_all(out_dir)?;
	fs::create_dir_all(out_dir)?;
	let mut plugins = vec![];
	for plugin in inventory::iter::<CooraPluginBindings> {
		let plugin: &CooraPluginBindings = plugin;
		plugins.push(plugin);
		let (name, file) = write_file(plugin);
		fs::write(name, file)?;
	}
	if plugins.len() == 0 {
		return Err(anyhow!("No plugins found.."));
	}
	Ok(plugins)
}


pub fn write_index<T>(
	plugins: Vec<&CooraPluginBindings>,
	out_dir: &PathBuf,
	write_line: T,
) -> Result<()>
where
	T: Fn(&&CooraPluginBindings) -> String,
{
	let index_str: Vec<String> = plugins.iter().map(write_line).collect();
	let index_str = index_str.join("\n");
	fs::write(out_dir, index_str)?;
	Ok(())
}


#[rustfmt::skip]
pub fn export_ts() -> Result<()> {
	let out = PathBuf::from(EXPORT_ROOT).join("typescript");
	let plugins = export_and_collect(&out, |plugin| {
		(out.join(plugin.name.to_case(Case::Camel)).with_extension("ts"),
		plugin.typescript_bindings,)
	})?;
	write_index(plugins, &out.join("index.ts"), |plugin| {
		format!("export * from \"./{}\";", plugin.name.to_case(Case::Camel))
	})?;
	Ok(())
}
#[rustfmt::skip]
pub fn export_rs() -> Result<()> {
	let out = PathBuf::from(EXPORT_ROOT).join("rust");
	let plugins = export_and_collect(&out, |plugin| {
		(out.join(plugin.name.to_case(Case::Snake)).with_extension("rs"),
		plugin.rust_bindings)
	})?;
	write_index(plugins, &out.join("mod.rs"), |plugin| {
		format!("pub mod {};", plugin.name.to_case(Case::Snake))
	})?;
	Ok(())
}
