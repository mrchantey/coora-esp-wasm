// use std::sync::{Arc, Mutex};

use std::sync::{Arc, Mutex};

use anyhow::Result;
use coora_engine::{SketchInstance, WasmApp, WasmEngine};
use embedded_svc::http::server::registry::Registry;
// use coora_engine::{SketchInstance, WasmEngine};
// use embedded_svc::http::server::registry::Registry;
use esp_idf_svc::wifi::EspWifi;

use crate::*;

pub struct SketchServer {}

impl SketchServer {
    pub fn run(store: &Store, wifi: &mut EspWifi) -> Result<()> {
        let wifi = wifi::WifiClient::from_store_or_ap(store, wifi)?;
        let mut server = wifi.start_server(store)?;
        let mut engine = WasmEngine::new();
        let (mut time, mut leds) = default_peripherals()?;

        let mut app = WasmApp::new(&mut engine, 0);
        app.add_plugin(&mut leds)?;
        app.add_plugin(&mut time)?;
        app.build(&mut engine);

        let mut sketch = SketchInstance::new(&mut app);

        let buffer = Arc::new(Mutex::new(SketchBuffer::new()));
        // let engine = Arc::clone(&sketch);
        let buffer2 = Arc::clone(&buffer);
        server.handle_post("/sketch", move |mut request, response| {
            let mut buffer = buffer2.lock().unwrap();
            buffer.from_request(&mut request)?;
            println!("\nsketch received! {}b", buffer.len);

            response.ok()?;
            Ok(())
        })?;
        loop {
            let mut buffer = buffer.lock().unwrap();
            if buffer.dirty {
                let mut app = WasmApp::new(&mut engine, 0);
                app.add_plugin(&mut leds)?;
                app.add_plugin(&mut time)?;
                app.build_with_wasm(&mut engine, &buffer.buffer[..buffer.len]);

                sketch = SketchInstance::new(&mut app);
                buffer.dirty = false;
            }
            sketch.run();
            utility::sleep_ms(16);
        }
    }
}
