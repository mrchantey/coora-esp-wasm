use anyhow::Result;
use coora_target_esp32::{
    wifi::{get_wifi, WifiFallbackClient},
    *,
};

fn main() -> Result<()> {
    //good credentials
    {
        let nvs = take_nvs()?;
        let mut wifi = get_wifi(&nvs)?;
        let mut client = WifiFallbackClient::new(&mut wifi, secret::SSID, secret::PASSWORD)?;
        client.check_status_sync(&mut wifi)?;
        assert!(!client.client.is_none(), "Expected client");
    }
    //bad credentials
    {
        let nvs = take_nvs()?;
        let mut wifi = get_wifi(&nvs)?;
        let mut client = WifiFallbackClient::new(&mut wifi, secret::SSID, "bad password")?;
        client.check_status_sync(&mut wifi)?;
        assert!(!client.access_point.is_none(), "Expected ap");
    }
    println!("ok!");
    Ok(())
}
