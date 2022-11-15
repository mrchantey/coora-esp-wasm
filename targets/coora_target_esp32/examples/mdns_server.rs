use anyhow::Result;
use coora_target_esp32::*;
//not yet released...
// use esp_idf_svc::mdns;

fn main() -> Result<()> {
    let wifi = wifi::Connection::new(secret::SSID, secret::PASSWORD)?;
    let _server = wifi.start_server()?;
    // mdns::
    println!("howdy!");
    utility::sleep_forever();
}
