use crate as coora_engine;
use crate::*;

#[coora_export]
pub trait Adder {
	fn add(a: i32, b: i32) -> i32;
}
