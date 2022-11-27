#![allow(non_snake_case)]
use crate as coora_engine;
use crate::*;

#[coora_export]
pub trait MemoryTest {
	fn printHello();
	//fix this name
	fn getString() -> u32;
	fn _size() -> u32;
	fn _load(ptr: i32) -> i32;
	fn _store(ptr: u32, val: i32) -> ();
}
