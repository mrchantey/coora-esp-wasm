use crate::*;
use crate as coora_engine;

#[coora_export]
pub trait Sketchy{
	fn run();
	//fix this name
	fn _millis()->u64;
}
