use anyhow::Result;
use coora_engine::*;
use coora_target_esp32::{
    utility::print_free_heap,
    wifi::{get_wifi, WifiFallbackClient},
    *,
};

fn main() -> Result<()> {
    print_free_heap("start");
    let mut esp32_imports = take_esp32_imports()?;
    print_free_heap("with store");
    let (nvs, mut store) = take_nvs_store()?;
    for i in 0..100 {
        println!("ATTEMPT - {i}");
        print_free_heap("restart attempt");
        let mut app = WasmApp::new();
        app.link_memory()?
            .add_plugin(&mut esp32_imports)?
            .add_plugin(&mut StdImports)?
            .build()?;
        let mut sketch = SketchInstance::new(&mut app);
        sketch.start();
        sketch.update();
        print_free_heap("with sketch");
        let mut wifi = get_wifi(&nvs)?;
        let mut client = WifiFallbackClient::new_from_store(&mut wifi, &mut store)?;
        client.check_status_sync(&mut wifi)?;
        let _server = wifi.start_server(&mut store)?;
        print_free_heap("with wifi");
        // print_free_heap("with server");
        // utility::sleep_ms(2000);
    }
    // Ok(())
    utility::sleep_forever();
}
