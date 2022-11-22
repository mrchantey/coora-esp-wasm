use anyhow::Result;
use coora_engine::{SketchInstance, WasmApp};
use embedded_svc::http::server::registry::Registry;
use esp_idf_svc::{http::server::EspHttpServer, wifi::EspWifi};
use std::sync::{Arc, Mutex};

use crate::{utility::b_to_kb, wifi::get_wifi, *};

const RESTART_FROM_NVS: bool = false;

fn run_wifi(
    sketch_buffer: Arc<Mutex<SketchBuffer>>,
    nvs: &Nvs,
    store: &mut Store,
) -> Result<(EspWifi, EspHttpServer)> {
    let mut wifi = get_wifi(nvs)?;
    let _wifi = wifi::WifiClient::from_store_or_ap(store, &mut wifi)?;
    let mut server = _wifi.start_server(store)?;

    let store = Arc::clone(store);
    server.handle_post("/sketch", move |mut request, response| {
        let mut buffer = sketch_buffer.lock().unwrap();
        buffer.from_request(&mut request)?;
        //TODO optionally dont nvs and attempt no restart
        if RESTART_FROM_NVS {
            buffer.set_nvs(&store)?;
        }
        println!("\nSKETCH received! {}", b_to_kb(buffer.len));
        response.ok()?;
        Ok(())
    })?;
    Ok((wifi, server))
}

pub fn run_sketch_server() -> Result<()> {
    let (nvs, mut store) = take_nvs_store()?;
    let (mut time, mut leds, mut console) = default_peripherals()?;
    let buffer = Arc::new(Mutex::new(SketchBuffer::from_nvs_or_default(&store)));
    'main: loop {
        // let buffer = Arc::new(Mutex::new(SketchBuffer::from_nvs_or_default(&store)));
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
        sketch.start();

        let _wifi = run_wifi(Arc::clone(&buffer), &nvs, &mut store)?;
        loop {
            if buffer.lock().unwrap().dirty {
                if RESTART_FROM_NVS {
                    break 'main; //break main for full restart
                } else {
                    break;
                }
            }
            utility::sleep_ms(16);
            sketch.run();
        }
        utility::sleep_ms(500); //allow for return ok message
        println!("SKETCH - reloading..");
    }
    println!("SKETCH - restarting..");
    // utility::sleep_ms(2000); //allow send ok message
    utility::restart();
}
