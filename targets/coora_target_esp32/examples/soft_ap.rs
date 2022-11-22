use anyhow::Result;
use coora_target_esp32::{wifi::get_wifi, *};

fn main() -> Result<()> {
    let (nvs, store) = take_nvs_store()?;
    let mut wifi = get_wifi(&nvs)?;
    let wifi = wifi::WifiAccessPoint::named_from_store(&store, &mut wifi)?;
    let _server = wifi.start_server(&store)?;
    utility::sleep_forever();
}
