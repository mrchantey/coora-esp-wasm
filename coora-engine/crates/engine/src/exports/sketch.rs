use crate as coora_engine;
use crate::*;

#[coora_export]
pub trait Sketch {
	fn start();
	fn run();
	fn ping() -> u64;
}
