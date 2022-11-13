#![allow(dead_code)]
#![no_std]
// uncomment ONE of these to run
// mod hello_led;
// mod hello_world;

#[cfg(not(test))]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    unreachable!()
}

// #[no_mangle]
// pub extern "C" fn hello() {
//     unsafe {
//         let a = 2;
//         let b = 10;
//         // howdy(a + b);
//     }
// }
#[no_mangle]
pub extern "C" fn add(a: u64, b: u64) -> u64 {
    a + b
}
