use anyhow::Result;
use coora_target_esp32::{wifi::get_wifi, *};

fn main() -> Result<()> {
    let store = StoreBuilder::take()?;
    let mut wifi = get_wifi(&store)?;
    SketchServer::run(&store.store, &mut wifi)?;
    Ok(())
}
