use anyhow::Result;
use coora_target_esp32::*;


fn main() -> Result<()> {
	SketchServer::run()?;
	// sketch.run();
	// // Arc::clone(&leds).lock().unwrap().show();
	// println!("sketch ok!");
	Ok(())
}
