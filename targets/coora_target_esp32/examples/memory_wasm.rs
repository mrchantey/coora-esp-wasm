use std::sync::Arc;

use anyhow::Result;
use coora_engine::{SketchInstance, WasmApp};
use coora_target_esp32::{utility::sleep_forever, wifi::get_wifi, *};
use wasmi::*;

fn main() -> Result<()> {
    // utility::print_free_heap("nothing"); //276
    let mut store = NvsStore::new()?;
    let mut _wifi = get_wifi(&store)?;
    let (mut time, mut leds, mut console) = default_peripherals()?;
    // app.build_with_wasm(WasmApp::wasm_hello_world())?;
    // utility::print_free_heap(); //231
    {
        let mut app = WasmApp::new();
        app.link_memory()?
            .add_plugin(&mut leds)?
            .add_plugin(&mut time)?
            .add_plugin(&mut console)?;
        app.build()?;
        let mut sketch = SketchInstance::new(&mut app);
        sketch.start();
        sketch.run();
        app.instance = None;
    }
    {
        let mut app = WasmApp::new();
        app.link_memory()?
            .add_plugin(&mut leds)?
            .add_plugin(&mut time)?
            .add_plugin(&mut console)?;
        app.build()?;
        let mut sketch = SketchInstance::new(&mut app);
        sketch.start();
        sketch.run();
        // utility::print_free_heap();
        app.instance = None;
        // utility::print_free_heap();
    }
    {
        // utility::print_free_heap();
        let wifi = wifi::WifiClient::from_store_or_ap(&mut store.store, &mut _wifi)?;
        let _server = wifi.start_server(&mut store.store)?;
        // utility::print_free_heap();
    }
    // Ok(())
    sleep_forever();
}
