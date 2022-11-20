use crate as coora_engine;
use crate::*;

#[coora_import]
pub trait Console {
	fn log(&mut self, val: &str);
}

pub struct StdConsole {}

impl Console for StdConsole {
	fn log(&mut self, val: &str) {
		println!("{val}");
	}
}
