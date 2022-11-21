use anyhow::Result;
use coora_target_esp32::*;
use embedded_svc::storage::*;

fn main() -> Result<()> {
    let store1 = NvsStore::new()?;
    let mut store1 = store1.store.lock().unwrap();

    let key = "foobar";
    if store1.contains(key)? == false {
        println!("key not found, setting & restarting...");
        let _ = store1.put_raw(key, &[0, 1, 2, 3][..])?;
        utility::sleep_ms(2000);
        utility::restart();
    }

    let mut size = 0;
    let mut buf = [0; 16];
    let result = store1.get_raw(key, &mut buf)?;
    if let Some((_, new_size)) = result {
        size = new_size
    }

    println!("key found, value: {:?}, removing...", &buf[..size]);
    store1.remove(key)?;
    Ok(())
}
