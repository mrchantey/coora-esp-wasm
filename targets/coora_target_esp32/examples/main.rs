// use anyhow::Result;
use coora_target_esp32::*;

fn main() {
    run_sketch().unwrap_or_else(|_| {
        panic!("failed..");
    });
}
