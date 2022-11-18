use crate::CooraPluginBindings;
use anyhow::{anyhow, Result};
use fs_extra::dir::CopyOptions;
use std::{fs, path::PathBuf};

pub const EXPORT_ROOT: &str = "target/bindings";

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


pub fn copy_bindings(src: PathBuf, dst: PathBuf) -> Result<()> {
	fs::remove_dir_all(&dst)?;
	fs::create_dir_all(&dst)?;
	fs_extra::dir::copy(&src, &dst, &CopyOptions::new())?;
	Ok(())
}
