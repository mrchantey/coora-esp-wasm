use anyhow::Result;
use coora_target_esp32::{utility::set_favourite_log_level, wifi::get_wifi, *};
use embedded_svc::http::Status;
use esp_idf_svc::http::client::EspHttpClient;

fn main() -> Result<()> {
    set_favourite_log_level();
    //sync
    {
        let nvs = take_nvs()?;
        let mut wifi = get_wifi(&nvs)?;

        let wifi_client = wifi::WifiClient::new(&mut wifi, secret::SSID, secret::PASSWORD)?;
        let settings = wifi_client.check_status_sync(&wifi)?;
        let mut http = EspHttpClient::new_https()?;
        println!("SYNC OK! ip: {}", settings.ip);

        assert!(http.get("http://httpbin.org/get")?.status() == 200);
        assert!(http.get("https://httpbin.org/get")?.status() == 200);
        let result = http.post("https://httpbin.org/post", b"howdy doody")?;
        assert!(result.status() == 200);
        // let len = result.read_bytes()
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
        let mut http = EspHttpClient::new_https()?;

        http.get("http://example.com")?;
        println!("HTTP OK!");
        http.get("https://espressif.com")?;
        println!("HTTPS OK!");
    }
    Ok(())
}
