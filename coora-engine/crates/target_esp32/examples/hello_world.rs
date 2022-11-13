use coora_target_esp32::utility;
use esp_idf_sys as _;
// esp_idf_sys::link_patches();
fn main() {
	let str = String::from("hello cool world!");
	loop {
		println!("{}", str);
		utility::sleep_ms(1000);
	}
}
