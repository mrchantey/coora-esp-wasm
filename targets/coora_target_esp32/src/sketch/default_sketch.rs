use crate::*;
use anyhow::Result;
use coora_engine::*;

pub fn default_sketch() -> Result<SketchInstance> {
    // let mut esp32_imports = take_esp32_imports()?;
    let device = IDFDevice::new();
    let led_pin = device.peripherals.pins.gpio7.into_output()?;
    let channel = device.peripherals.rmt.channel0;
    let mut leds = led_strip_rgbw!(led_pin, channel, 6)?.as_shared();

    let mut app = WasmApp::new();
    // app.add_plugin(&mut esp32_imports)?
    app.add_plugin(&mut leds)?
        .add_plugin(&mut StdImports)?
        .build()?;

    let sketch = SketchInstance::new(&mut app);

    Ok(sketch)
}
