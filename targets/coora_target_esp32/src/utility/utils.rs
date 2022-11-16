use esp_idf_sys::esp_restart;
use std::time::Duration;

pub fn sleep_ms(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}
pub fn sleep_forever() -> ! {
    loop {
        sleep_ms(16); //~60fps
    }
}
pub fn restart() -> ! {
    unsafe {
        esp_restart();
    }
    //...actually never reached
    sleep_forever();
}
