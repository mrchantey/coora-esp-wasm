
use crate::*;
use anyhow::Result;
use coora_engine::*;
use esp_idf_hal::{
    gpio::{Gpio7, Output},
    rmt::CHANNEL0,
};

pub fn default_sketch() -> Result<SketchInstance> {
    let (mut time, mut leds) = default_peripherals()?;
    let mut engine = WasmEngine::new();

    let mut app = WasmInstanceBuilder::new(&mut engine, 0);
    app.bind(&mut leds).bind(&mut time);
    let app = SketchInstance::build_with_default_sketch(&mut engine, app);

    // let instance = SketchInstance::from_default(&leds);
    Ok(app)
}

pub fn default_peripherals() -> Result<(
    DeorphanedTime<StdTime>,
    DeorphanedLedStrip<LedStripRGBW<Gpio7<Output>, CHANNEL0, 6, 193>>,
)> {
    let device = IDFDevice::new();
    let led_pin = device.peripherals.pins.gpio7.into_output().unwrap();
    let channel = device.peripherals.rmt.channel0;
    let leds = led_strip_rgbw!(led_pin, channel, 6)?.as_shared();
    let time = StdTime::new().as_shared();
    Ok((time, leds))
}
