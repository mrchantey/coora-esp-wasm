// use anyhow::Result;
use coora_target_esp32::*;

fn main() {
    utility::set_favourite_log_level();
    run_sketch().unwrap_or_else(|_| {
        panic!("failed..");
    });
}
