use std::sync::Arc;

use anyhow::Result;
use coora_target_esp32::*;
use embedded_svc::storage::*;

fn main() -> Result<()> {
    println!("ok 0");
    let store1 = StoreBuilder::take()?;
    let store1 = store1.store;
    let store2 = Arc::clone(&store1);
    {
        let mut store1 = store1.lock().unwrap();

        println!("ok 1");
        let _ = store1.contains("foobar")?;
        store1.remove("foobar")?;
    }
    {
        let mut store2 = store2.lock().unwrap();
        let _ = store2.contains("foobar")?;
        println!("ok 2");
        store2.remove("foobar")?;
        println!("ok 3");
    }
    Ok(())
}
