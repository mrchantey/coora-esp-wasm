use anyhow::Result;
use coora_target_esp32::{wifi::*, *};

fn main() -> Result<()> {
    //sync
    {
        let nvs = take_nvs()?;
        let mut wifi = get_wifi(&nvs)?;

        let client = WifiAccessPoint::new_default(&mut wifi)?;
        let settings = client.check_status_sync(&wifi)?;
        println!("SYNC OK! ip: {}", settings.ip);
    }
    //async
    {
        let nvs = take_nvs()?;
        let mut wifi = get_wifi(&nvs)?;

        let client = WifiAccessPoint::new_default(&mut wifi)?;
        loop {
            if let Some(settings) = client.check_status(&wifi)? {
                println!("ASYNC OK! ip: {}", settings.ip);
                break;
            }
        }
    }
    // utility::sleep_forever();
    Ok(())
}
