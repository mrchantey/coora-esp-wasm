use crate::*;
use anyhow::Result;
use convert_case::{Case, Casing};
use fs_extra::dir::CopyOptions;
use std::{fs, path::PathBuf};

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


pub fn move_files_ts() -> Result<()> {
	let bindings_dir = PathBuf::from(EXPORT_ROOT).join("typescript");
	let target_dir = PathBuf::new().join("../coora-app/packages/examples/src/bindings");
	fs::remove_dir_all(&target_dir)?;
	fs::create_dir_all(&target_dir)?;
	fs_extra::dir::copy(&bindings_dir, &target_dir, &CopyOptions::new())?;
	Ok(())
}
