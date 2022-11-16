use anyhow::Result;
use coora_target_esp32::{wifi::get_wifi, *};

fn main() -> Result<()> {
    let store = StoreBuilder::take()?;
    let mut wifi = get_wifi(&store)?;
    let wifi = wifi::WifiClient::new(&mut wifi, secret::SSID, secret::PASSWORD)?;
    let _server = wifi.start_server(&store.store)?;
    println!("howdy!");
    utility::sleep_forever();
}
