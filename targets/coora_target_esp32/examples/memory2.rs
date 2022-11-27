use anyhow::Result;
use coora_target_esp32::{self as _, utility::sleep_ms};

const PAGE_SIZE: usize = 65536;
// const NUM_PAGES: usize = 8; //yup 8 fails
const NUM_PAGES: usize = 4;
const TOTAL_SIZE: usize = PAGE_SIZE * NUM_PAGES;
fn main() -> Result<()> {
    sleep_ms(1000);

    let arr = [7 as u8; TOTAL_SIZE];
    println!("dont panic, i just allocated {}b", arr.len());
    println!("some value is {}", arr[100]);
    // for i in 0..TOTAL_SIZE {
    // 	print!("{}", arr[i]);
    // }
    Ok(())
}
