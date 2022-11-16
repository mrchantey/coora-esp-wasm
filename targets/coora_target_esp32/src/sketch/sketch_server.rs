use std::sync::{Arc, Mutex};

use anyhow::Result;
use coora_engine::{SketchInstance, WasmEngine};
use embedded_svc::http::server::registry::Registry;
use esp_idf_svc::wifi::EspWifi;

use crate::*;

pub struct SketchServer {}

impl SketchServer {
    pub fn run(store: &Store, wifi: &mut EspWifi) -> Result<()> {
        let wifi = wifi::WifiClient::from_store_or_ap(store, wifi)?;
        let mut server = wifi.start_server(store)?;

        let mut engine = WasmEngine::new();
        let leds = default_peripherals()?;
        let mut sketch = SketchInstance::from_default_with_engine(&mut engine, &leds);

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
                // sketch.instance.instance.
                // let mut engine = WasmEngine::new();
                sketch = SketchInstance::new_with_engine(
                    &mut engine,
                    &buffer.buffer[..buffer.len],
                    &leds,
                );
                buffer.dirty = false;
            }
            sketch.run();
            utility::sleep_ms(16);
        }
    }
}
