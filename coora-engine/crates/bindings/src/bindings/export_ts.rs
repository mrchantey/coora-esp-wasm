use crate::*;
use anyhow::Result;
use convert_case::{Case, Casing};
use std::path::PathBuf;

#[rustfmt::skip]
pub fn export_ts() -> Result<()> {
	let out = PathBuf::from(EXPORT_ROOT).join("typescript");
	let plugins = export_and_collect(&out, |plugin| {
		(out.join(plugin.name.to_case(Case::Camel)).with_extension("ts"),
		plugin.typescript_bindings,)
	})?;
	write_index(plugins, &out.join("index.ts"), |plugin| {
		let name = plugin.name.to_case(Case::Camel);
// invalid asc: export * as {name} from \'./{name}\'
		format!("import * as {name} from './{name}'\nexport {{ {name} as {name} }}")
	})?;
	move_files_ts()?;
	Ok(())
}

pub fn move_files_ts() -> Result<()> {
	let src = PathBuf::from(EXPORT_ROOT).join("typescript");
	let dst = PathBuf::new().join("../coora-app/packages/examples/src/bindings");
	copy_bindings(src, dst)
}
