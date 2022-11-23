use anyhow::Result;
use coora_target_esp32::{wifi::get_wifi, *};

fn main() -> Result<()> {
    //sync
    {
        let nvs = take_nvs()?;
        let mut wifi = get_wifi(&nvs)?;

        let client = wifi::WifiClient::new(&mut wifi, secret::SSID, secret::PASSWORD)?;
        let settings = client.check_status_sync(&wifi)?;
				println!("SYNC OK! ip: {}", settings.ip);

        wifi.get("http://example.com")?;
        println!("HTTP OK!");
        wifi.get("https://espressif.com")?;
        println!("HTTPS OK!");
    }
    //async
    {
        let nvs = take_nvs()?;
        let mut wifi = get_wifi(&nvs)?;

        let client = wifi::WifiClient::new(&mut wifi, secret::SSID, secret::PASSWORD)?;
        loop {
            if let Some(settings) = client.check_status(&wifi)? {
                println!("ASYNC OK! ip: {}", settings.ip);
                break;
            }
        }

        wifi.get("http://example.com")?;
        println!("HTTP OK!");
        wifi.get("https://espressif.com")?;
        println!("HTTPS OK!");
    }
    Ok(())
}
