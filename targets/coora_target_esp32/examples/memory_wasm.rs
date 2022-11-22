use anyhow::Result;
use coora_engine::{SketchInstance, WasmApp};
use coora_target_esp32::{utility::print_free_heap, wifi::get_wifi, *};

fn main() -> Result<()> {
    print_free_heap("start");
    let (mut time, mut leds, mut console) = default_peripherals()?;
    print_free_heap("with store");
    let (nvs, mut store) = take_nvs_store()?;
    for i in 0..100 {
        println!("ATTEMPT - {i}");
        print_free_heap("restart attempt");
        let mut app = WasmApp::new();
        app.link_memory()?
            .add_plugin(&mut leds)?
            .add_plugin(&mut time)?
            .add_plugin(&mut console)?;
        app.build()?;
        let mut sketch = SketchInstance::new(&mut app);
        sketch.start();
        sketch.run();
        print_free_heap("with sketch");
        let mut _wifi = get_wifi(&nvs)?;
        let wifi = wifi::WifiClient::from_store_or_ap(&mut store, &mut _wifi)?;
        let _server = wifi.start_server(&mut store)?;
        print_free_heap("with wifi");
        // print_free_heap("with server");
        // utility::sleep_ms(2000);
    }
    // Ok(())
    utility::sleep_forever();
}
