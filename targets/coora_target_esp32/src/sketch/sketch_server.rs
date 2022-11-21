use anyhow::Result;
use coora_engine::{SketchInstance, WasmApp};
use embedded_svc::http::server::registry::Registry;
use esp_idf_svc::{http::server::EspHttpServer, wifi::EspWifi};
use std::sync::{Arc, Mutex};

use crate::{
    utility::b_to_kb,
    wifi::{get_wifi, WifiClient},
    *,
};

fn run_wifi(
    sketch_buffer: Arc<Mutex<SketchBuffer>>,
    store: &mut Store,
    wifi: &mut EspWifi,
) -> Result<(WifiClient, EspHttpServer)> {
    let wifi = wifi::WifiClient::from_store_or_ap(store, wifi)?;
    let mut server = wifi.start_server(store)?;

    let store = Arc::clone(store);
    server.handle_post("/sketch", move |mut request, response| {
        let mut buffer = sketch_buffer.lock().unwrap();
        buffer.from_request(&mut request)?;
        //TODO optionally dont nvs and attempt no restart
        buffer.set_nvs(&store)?;
        println!("\nSKETCH received! {}", b_to_kb(buffer.len));
        response.ok()?;
        Ok(())
    })?;
    Ok((wifi, server))
}

pub fn run_sketch_server() -> Result<()> {
    // let a = *nvs.store.lock().unwrap();
    // a.
    'main: loop {
        let mut nvs = NvsStore::new()?;
        let (mut time, mut leds, mut console) = default_peripherals()?;
        let mut wifi = get_wifi(&nvs)?;
        let buffer = Arc::new(Mutex::new(SketchBuffer::from_nvs_or_default(&nvs.store)));
        let mut app = WasmApp::new();
        {
            //releasable buffer
            let mut buffer = buffer.lock().unwrap();
            println!("SKETCH - building.. {}", b_to_kb(buffer.len));
            buffer.dirty = false;
            app.add_plugin(&mut leds)?
                .add_plugin(&mut time)?
                .add_plugin(&mut console)?
                .build_with_wasm(&buffer.buffer[..buffer.len])?;
        }
        let mut sketch = SketchInstance::new(&mut app);
        let _wifi = run_wifi(Arc::clone(&buffer), &mut nvs.store, &mut wifi)?;
        // _wifi.1.
        sketch.start();
        loop {
            if buffer.lock().unwrap().dirty {
                break 'main;
            }
            sketch.run();
            utility::sleep_ms(16);
        }
    }
    println!("SKETCH - restarting..");
    // utility::sleep_ms(2000); //allow send ok message
    utility::restart();
}
