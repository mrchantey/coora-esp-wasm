use anyhow::Result;
use coora_target_esp32::{mqtt::*, wifi::*, *};

fn main() -> Result<()> {
    let (nvs, store) = take_nvs_store()?;
    let mut wifi = get_wifi(&nvs)?;
    let mut client = WifiFallbackClient::new_from_store(&mut wifi, &store)?;
    client.check_status_sync(&mut wifi)?;
    let mut mqtt = MqttClient::new()?;

    mqtt.subscribe()?;
    // client.publish(b"hello world!")?;

    // client.client.
    // let mut wifi = get_wifi(&store)?;
    // let wifi = wifi::WifiClient::new(&mut wifi, secret::SSID, secret::PASSWORD)?;
    // let _server = wifi.start_server(&store.store)?;
    // println!("howdy!");
    utility::sleep_forever();
}
