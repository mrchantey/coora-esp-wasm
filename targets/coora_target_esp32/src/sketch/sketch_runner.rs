use anyhow::Result;
use coora_engine::*;
use std::sync::{Arc, Mutex};

use crate::{utility::b_to_kb, *};

#[derive(PartialEq, Copy, Clone)]
pub enum SketchReloadMode {
    RestartDevice,
    RestartWifi,
}

pub fn run_sketch() -> Result<()> {
    let reload_mode = SketchReloadMode::RestartWifi;
    let (nvs, mut store) = take_nvs_store()?;
    let mut esp32_imports = take_esp32_imports()?;
    let buffer = Arc::new(Mutex::new(SketchBuffer::from_nvs_or_default(&store)));
    'main: loop {
        let mut app = WasmApp::new();
        {
            //releasable buffer
            let mut buffer = buffer.lock().unwrap();
            println!("SKETCH - building.. {}", b_to_kb(buffer.len));
            buffer.dirty = false;
            app.add_plugin(&mut esp32_imports)?
                .add_plugin(&mut StdImports)?
                .build_with_wasm(&buffer.buffer[..buffer.len])?;
        }
        let mut sketch = SketchInstance::new(&mut app);
        sketch.start();

        let mut server = SketchServer::new(Arc::clone(&buffer), &nvs, &mut store, reload_mode)?;
        loop {
            if buffer.lock().unwrap().dirty {
                if reload_mode == SketchReloadMode::RestartDevice {
                    break 'main;
                } else {
                    break;
                }
            }
            sketch.update();
            server.update()?;
            utility::sleep_ms(16);
        }
        utility::sleep_ms(500); //allow for return ok message
        println!("SKETCH - reloading..");
    }
    println!("SKETCH - restarting..");
    // utility::sleep_ms(2000); //allow send ok message
    utility::restart();
}
