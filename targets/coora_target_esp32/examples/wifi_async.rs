use anyhow::Result;
use coora_target_esp32::{
    wifi::{get_wifi, EspWifiExt},
    *,
};

fn main() -> Result<()> {
    let nvs = take_nvs()?;
    let mut wifi = get_wifi(&nvs)?;
    wifi.set_config(secret::SSID, secret::PASSWORD)?;
    // let wifi = wifi::WifiClient::new(&mut wifi, secret::SSID, secret::PASSWORD)?;

    loop {
        println!("connecting..");
        if let Some(_settings) = wifi.check_status() {
            break;
        }
    }
    wifi.get("http://example.com")?;
    println!("HTTP OK!");
    wifi.get("https://espressif.com")?;
    println!("HTTPS OK!");
    // sleep_forever();
    Ok(())
}
