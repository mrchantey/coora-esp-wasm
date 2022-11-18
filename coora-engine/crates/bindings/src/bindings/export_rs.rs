use crate::*;
use anyhow::Result;
use convert_case::{Case, Casing};
use std::path::PathBuf;

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
