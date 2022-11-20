use anyhow::Result;
use coora_engine::build_mem_test;
use coora_target_esp32 as _;

fn main() -> Result<()> {
    let mut mem_test = build_mem_test()?;
    mem_test.printHello();
    mem_test.printHello();
    mem_test.printHello();
    Ok(())
}
