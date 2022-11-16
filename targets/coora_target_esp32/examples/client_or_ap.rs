use anyhow::Result;
use coora_target_esp32::{wifi::get_wifi, *};

fn main() -> Result<()> {
    let store = StoreBuilder::take()?;
    let mut wifi = get_wifi(&store)?;
    let wifi = wifi::WifiClient::from_store_or_ap(&store.store, &mut wifi)?;
    let _server = wifi.start_server(&store.store)?;
    utility::sleep_forever();
}
