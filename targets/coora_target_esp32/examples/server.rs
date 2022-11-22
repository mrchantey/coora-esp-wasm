use anyhow::Result;
use coora_target_esp32::{wifi::get_wifi, *};

fn main() -> Result<()> {
    let (nvs, store) = take_nvs_store()?;
    let mut wifi = get_wifi(&nvs)?;
    let wifi = wifi::WifiClient::new(&mut wifi, secret::SSID, secret::PASSWORD)?;
    let _server = wifi.start_server(&store)?;
    println!("howdy!");
    utility::sleep_forever();
}
