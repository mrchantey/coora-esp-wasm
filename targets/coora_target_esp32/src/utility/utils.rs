#![allow(non_upper_case_globals)]
use esp_idf_sys::*;
use std::time::Duration;
//https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/misc_system_api.html
pub fn sleep_ms(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}

pub fn b_to_kb<T>(b: T) -> String
where
    T: MyInt,
{
    format!("{:.2} KB", b.to_f32_lossy() / 1000.)
}

pub fn print_free_heap(prefix: &str) {
    println!(
        "HEAP - {prefix} - total: {}, internal: {}",
        b_to_kb(free_heap_size()),
        b_to_kb(free_internal_heap_size())
    );
}

pub fn free_heap_size() -> u32 {
    unsafe { esp_get_free_heap_size() }
}
pub fn free_internal_heap_size() -> u32 {
    unsafe { esp_get_free_internal_heap_size() }
}

pub fn was_ok_reset() -> bool {
    match reset_reason() {
        esp_reset_reason_t_ESP_RST_DEEPSLEEP => true,
        esp_reset_reason_t_ESP_RST_SW => true,
        esp_reset_reason_t_ESP_RST_POWERON => true,
        // esp_reset_reason_t_ESP_RST_STDIO => true, --any number?
        _ => false,
    }
}

pub fn reset_reason() -> esp_reset_reason_t {
    unsafe { esp_reset_reason() }
}

pub fn sleep_forever() -> ! {
    // foo::res
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

pub trait MyInt {
    fn to_f32_lossy(self) -> f32;
}

impl MyInt for i32 {
    fn to_f32_lossy(self) -> f32 {
        self as f32
    }
}
impl MyInt for u32 {
    fn to_f32_lossy(self) -> f32 {
        self as f32
    }
}

impl MyInt for f32 {
    fn to_f32_lossy(self) -> f32 {
        self
    }
}
impl MyInt for usize {
    fn to_f32_lossy(self) -> f32 {
        self as f32
    }
}
