use coora_engine::build_hello_world;
use coora_target_esp32::*;
fn main() {
    let mut _instance = build_hello_world().unwrap();

    // let add = instance.get_export::<(u64, u64), u64>("add");
    // let result = add.call(&mut instance.store, (1, 2)).unwrap();

    // println!("result is {}", result);
    utility::sleep_forever();
}
