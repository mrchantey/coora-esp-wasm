use anyhow::Result;
use coora_target_esp32::{utility::print_free_heap, *};
use embedded_svc::{
    http::server::registry::Registry,
    storage::StorageBase,
    wifi::{ClientConnectionStatus, ClientIpStatus, ClientStatus, Configuration, Wifi},
};

use log::{set_max_level, LevelFilter};
use std::sync::{Arc, Mutex};

fn main() -> Result<()> {
    set_max_level(LevelFilter::Debug);
    print_free_heap("nothing");
    for i in 0..2 {
        print_free_heap("starting again..");
        println!("ATTEMPT - {i}");

        let nvs = Arc::new(esp_idf_svc::nvs::EspDefaultNvs::new()?);
        let storage = esp_idf_svc::nvs_storage::EspNvsStorage::new_default(
            Arc::clone(&nvs),
            "default",
            true,
        )?;
        let store = Arc::new(Mutex::new(storage));

        let netif_stack = Arc::new(esp_idf_svc::netif::EspNetifStack::new()?);
        let sys_look_stack = Arc::new(esp_idf_svc::sysloop::EspSysLoopStack::new()?);
        // let nvs = Arc::new(EspDefaultNvs::new()?);
        let mut wifi =
            esp_idf_svc::wifi::EspWifi::new(netif_stack, sys_look_stack, Arc::clone(&nvs))?;
        wifi.set_configuration(&Configuration::Client(
            embedded_svc::wifi::ClientConfiguration {
                ssid: secret::SSID.into(),
                password: secret::PASSWORD.into(),
                ..Default::default()
            },
        ))?;

        wifi.wait_status_with_timeout(wifi::TIMEOUT_DURATION, |s| !s.is_transitional())
            .map_err(|e| anyhow::anyhow!("WIFI CLIENT - timeout: {:?}", e))?;

        let status = wifi.get_status();

        // println!("WIFI CLIENT - status: {:?}", status);

        if let ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
            client_settings,
        ))) = status.0
        {
            println!(
                "\n\nWIFI CLIENT - connected\nWIFI CLIENT - IP: {:?}\n\n",
                client_settings.ip
            );
        }
        let mut _server = esp_idf_svc::http::server::EspHttpServer::new(
            &esp_idf_svc::http::server::Configuration::default(),
        )?;
        print_free_heap("with server");
        let store1 = Arc::clone(&store);
        _server.handle_get("/remove-key", move |_request, response| {
            let mut store = store1.lock().unwrap();
            store.remove("some_key")?;
            response.ok()?;
            Ok(())
        })?;
        // store1.lock();
    }
    print_free_heap("nothing");
    Ok(())
}
