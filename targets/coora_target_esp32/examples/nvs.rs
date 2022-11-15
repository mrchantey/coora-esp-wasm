use anyhow::Result;
use coora_target_esp32::*;
use embedded_svc::storage::*;
use esp_idf_svc::{nvs::*, nvs_storage::*};
use std::sync::Arc;

fn main() -> Result<()> {
    let a = EspDefaultNvs::new()?;
    let a = Arc::new(a);
    let mut storage = EspNvsStorage::new_default(a, "test", true)?;
    // let success = storage.put_raw("foobar", &[0, 1, 2, 3][..])?;
    let success = false;
    let contains = storage.contains("foobar")?;
    let mut size = 0;
    let mut buf = [0; 16];
    let result = storage.get_raw("foobar", &mut buf)?;
    if let Some((_, new_size)) = result {
        size = new_size
    }
    println!(
        "success: {}, contains: {}, buf: {:?}",
        success,
        contains,
        &buf[..size]
    );
    utility::sleep_forever();
}
