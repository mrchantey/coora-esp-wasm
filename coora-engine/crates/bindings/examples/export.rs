use anyhow::Result;
pub use coora_bindings::*;
pub use coora_engine::*;

pub fn main() -> Result<()> {
	export_ts()?;
	export_rs()?;
	Ok(())
}
