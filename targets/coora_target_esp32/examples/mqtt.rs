use anyhow::Result;
use coora_target_esp32::{mqtt::*, wifi::*, *};

fn main() -> Result<()> {
    // let store = NvsStore::new()?;
    let store = NvsStore::new()?;
    let mut wifi = get_wifi(&store)?;
    let _wifi = wifi::WifiClient::from_store_or_ap(&store.store, &mut wifi)?;

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
