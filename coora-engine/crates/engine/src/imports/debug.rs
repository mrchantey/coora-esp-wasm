use crate as coora_engine;
use crate::*;

#[coora_import]
pub trait Debug {
	fn log(&mut self, val: &str);
}
#[derive(Default)]
pub struct StdDebug {}


impl Debug for StdDebug {
	fn log(&mut self, val: &str) {
		println!("{val}");
	}
}
