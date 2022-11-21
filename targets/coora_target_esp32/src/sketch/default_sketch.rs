use crate::*;
use anyhow::Result;
use coora_engine::*;
use esp_idf_hal::{
    gpio::{Gpio7, Output},
    rmt::CHANNEL0,
};

pub fn default_sketch() -> Result<SketchInstance> {
    let (mut time, mut leds, mut console) = default_peripherals()?;

    let mut app = WasmApp::new();
    app.add_plugin(&mut leds)?
        .add_plugin(&mut time)?
        .add_plugin(&mut console)?
        .build()?;

    let sketch = SketchInstance::new(&mut app);

    Ok(sketch)
}

pub fn default_peripherals() -> Result<(
    DeorphanedTime<StdTime>,
    DeorphanedLedStrip<LedStripRGBW<Gpio7<Output>, CHANNEL0, 6, 193>>,
    DeorphanedConsole<StdConsole>,
)> {
    let device = IDFDevice::new();
    let led_pin = device.peripherals.pins.gpio7.into_output().unwrap();
    let channel = device.peripherals.rmt.channel0;
    let leds = led_strip_rgbw!(led_pin, channel, 6)?.as_shared();
    let time = StdTime::new().as_shared();
    let console = StdConsole {}.as_shared();
    Ok((time, leds, console))
}
