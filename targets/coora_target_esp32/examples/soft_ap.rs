use anyhow::Result;
use coora_target_esp32::{wifi::utility::get_wifi, *};

fn main() -> Result<()> {
    let store = StoreBuilder::take()?;
    let mut wifi = get_wifi(&store)?;
    let wifi = wifi::WifiAccessPoint::named_from_store(&store.store, &mut wifi)?;
    let _server = wifi.start_server(&store.store)?;
    utility::sleep_forever();
}
