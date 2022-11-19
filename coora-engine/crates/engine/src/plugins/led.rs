use crate::*;
use crate as coora_engine; //this is awkward


#[coora_import]
pub trait LedStrip {
	fn set_leds(&mut self, r: u32, g: u32, b: u32, w: u32);
	fn show(&mut self);
}
