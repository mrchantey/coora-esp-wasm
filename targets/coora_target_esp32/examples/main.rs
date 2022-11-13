use anyhow::Result;
use coora_target_esp32::*;

fn main() -> Result<()> {
    SketchServer::run()?;
    Ok(())
}
