use anyhow::Result;
use coora_target_esp32::{
    utility::print_free_heap,
    wifi::{get_wifi, WifiCredentials},
    *,
};
use embedded_svc::http::server::registry::Registry;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use log::{set_max_level, LevelFilter};
use std::sync::Arc;

fn main() -> Result<()> {
    set_max_level(LevelFilter::Debug);
    print_free_heap("nothing");
    for i in 0..2 {
        print_free_heap("starting again..");
        let (nvs, mut store) = take_nvs_store()?;
        print_free_heap("with store");
        println!("ATTEMPT - {i}");
        let mut _wifi = get_wifi(&nvs)?;
        let _wifi = wifi::WifiClient::from_store_or_ap(&mut store, &mut _wifi)?;
        // let _server = wifi.start_server(&mut store.store)?;
        let mut _server = EspHttpServer::new(&Configuration::default())?;
        print_free_heap("with server");
        let store1 = Arc::clone(&store);
        WifiCredentials::clear(&store1)?;

        _server.handle_get("/clear-wifi", move |_request, response| {
            // WifiCredentials::clear(&store1)?;
            response.ok()?;
            Ok(())
        })?;
        // _server.
    }
    print_free_heap("nothing");
    Ok(())
}
