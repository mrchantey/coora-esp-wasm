use crate as coora_engine;
use crate::*;

#[coora_export]
pub trait Sketch {
	fn start();
	fn update();
	fn ping() -> u64;
}
