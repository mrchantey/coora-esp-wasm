// use anyhow::Result;
use coora_target_esp32::{utility::set_esp_log_level, *};

fn main() {
    set_esp_log_level(log::LevelFilter::Warn);
    run_sketch().unwrap_or_else(|_| {
        panic!("failed..");
    });
}
