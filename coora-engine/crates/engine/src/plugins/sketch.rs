use crate::*;
use crate as coora_engine;

#[coora_export]
pub trait Sketch{
	fn run();
	//fix this name
	fn _millis()->u64;
}
