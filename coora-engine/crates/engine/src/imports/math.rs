use crate as coora_engine;
use crate::*;


#[coora_import]
pub trait Math {
	fn sin(&mut self, val: F32) -> F32;
}


pub struct StdMath;

impl StdMath {
	pub fn new() -> StdMath { StdMath {} }
}

impl Math for StdMath {
	fn sin(&mut self, val: F32) -> F32 { f32::sin(val.into()).into() }
}
