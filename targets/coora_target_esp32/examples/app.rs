use anyhow::Result;
use coora_target_esp32::*;

fn main() -> Result<()> {
    let mut sketch = default_sketch()?;
    sketch.start();
    sketch.run();
    // Arc::clone(&leds).lock().unwrap().show();
    println!("sketch ok!");
    Ok(())
}
