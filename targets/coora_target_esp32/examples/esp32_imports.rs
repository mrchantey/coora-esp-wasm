use std::sync::Arc;

use anyhow::Result;
use coora_target_esp32::{
    utility::{print_free_heap, sleep_forever},
    *,
};

fn main() -> Result<()> {
    // let mut _esp32_imports = take_esp32_imports()?;
    let device = IDFDevice::new();
    let led_pin = device.peripherals.pins.gpio7.into_output()?;
    let channel = device.peripherals.rmt.channel0;
    print_free_heap("before");
    let leds = led_strip_rgbw!(led_pin, channel, 24)?;
    print_free_heap("after");

    println!("ok1");
    // let leds = Mutex::new(leds);
    println!("ok2");
    let leds = Arc::new(leds);
    println!("ok3");
    println!("ok4");

    // let mut app = WasmApp::new();
    // app.add_plugin(&mut esp32_imports)?
    // app.add_plugin(&mut leds)?
    //     .add_plugin(&mut StdImports)?
    //     .build()?;

    // let sketch = SketchInstance::new(&mut app);
    // // println!("ok!");
    // print_free_heap("after 2");

    sleep_forever();
    // Ok(())
}
