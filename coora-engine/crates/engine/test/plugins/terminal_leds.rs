use coora_engine::*;


pub struct TerminalLeds {
	leds: Vec<rgb::RGBA8>,
}

impl TerminalLeds {
	pub fn new(size: usize) -> TerminalLeds {
		TerminalLeds {
			leds: vec![
				rgb::RGBA8 {
					r: 0,
					g: 0,
					b: 0,
					a: 0,
				};
				size
			],
		}
	}
}

impl LedStrip for TerminalLeds {
	fn set_leds(&mut self, r: u32, g: u32, b: u32, w: u32) {
		for mut led in &mut self.leds {
			led.r = r as u8;
			led.g = g as u8;
			led.b = b as u8;
			led.a = w as u8;
		}
	}
	fn show(&mut self) {
		for rgb::RGBA8 { r, g, b, a } in &self.leds {
			println!("r: {}\tg: {}\tb: {}\tw: {}", r, g, b, a);
		}
	}

}
