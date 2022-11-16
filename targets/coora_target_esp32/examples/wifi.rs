use anyhow::Result;
use coora_target_esp32::{wifi::get_wifi, *};

fn main() -> Result<()> {
    let store = StoreBuilder::take()?;
    let mut wifi = get_wifi(&store)?;
    let wifi = wifi::WifiClient::new(&mut wifi, secret::SSID, secret::PASSWORD)?;

    wifi.get("http://example.com")?;
    println!("HTTP OK!");
    wifi.get("https://espressif.com")?;
    println!("HTTPS OK!");
    Ok(())
}
