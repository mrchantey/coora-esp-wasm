use crate::*;
use crate::{led_strip_rgbw, IDFDevice};
use anyhow::Result;
use coora_engine::*;
use esp_idf_hal::{
    gpio::{Gpio7, Output},
    rmt::CHANNEL0,
};

use coora_engine::{Plugin, WasmApp};

pub fn take_esp32_imports(
) -> Result<DeorphanedLedStrip<LedStripRGBW<Gpio7<Output>, CHANNEL0, 6, 193>>> {
    // ) -> Result<Esp32Imports<DeorphanedLedStrip<LedStripRGBW<Gpio7<Output>, CHANNEL0, 24, 769>>>> {
    let device = IDFDevice::new();
    let led_pin = device.peripherals.pins.gpio7.into_output()?;
    let channel = device.peripherals.rmt.channel0;
    let leds = led_strip_rgbw!(led_pin, channel, 6)?.as_shared();

    Ok(leds)
    // Ok(Esp32Imports::new(leds))
}

// pub fn take_esp32_imports(
// ) -> Result<Esp32Imports<DeorphanedLedStrip<LedStripRGBW<Gpio7<Output>, CHANNEL0, 12, 385>>>> {
//     let device = IDFDevice::new();
//     let led_pin = device.peripherals.pins.gpio7.into_output()?;
//     let channel = device.peripherals.rmt.channel0;
//     let leds = led_strip_rgbw!(led_pin, channel, 12)?.as_shared();

//     Ok(Esp32Imports::new(leds))
// }
pub struct Esp32Imports<T>
where
    T: Plugin,
{
    leds: T,
}

impl<T> Esp32Imports<T>
where
    T: Plugin,
{
    pub fn new(leds: T) -> Esp32Imports<T> {
        Esp32Imports { leds }
    }
}

impl<T> Plugin for Esp32Imports<T>
where
    T: Plugin,
{
    fn bind(&mut self, app: &mut WasmApp) -> Result<()> {
        app.add_plugin(&mut self.leds)?;
        Ok(())
    }
}
