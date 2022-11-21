use anyhow::Result;
use coora_engine::*;
use coora_target_esp32::{self as _, utility::sleep_ms};

fn main() -> Result<()> {
    sleep_ms(1000);
    let mut mem_test = build_mem_test()?;
    mem_test.printHello();
    mem_test.printHello();
    mem_test.printHello();

    Ok(())
}
