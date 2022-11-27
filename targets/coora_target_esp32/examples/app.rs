use anyhow::Result;
use coora_target_esp32::*;

fn main() -> Result<()> {
    let mut sketch = default_sketch()?;
    sketch.start();
    println!("sketch ok!");
    loop {
        sketch.update();
        utility::sleep_ms(16);
    }
    // Arc::clone(&leds).lock().unwrap().show();
    // Ok(())
}
