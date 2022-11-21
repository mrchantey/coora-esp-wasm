use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use coora_engine::{SketchInstance, UserState, WasmApp};
use coora_target_esp32::*;
use wasmi::*;
/*
better if you throw out whole thing?


*/
fn main() -> Result<()> {
    let (mut time, mut leds, mut console) = default_peripherals()?;
    // let mut app = WasmApp::new();

    for i in 0..100 {
        println!("running {i}");
        // .build_with_wasm(&include_wasm!("../../", "hello_world")[..]);
        let mut app = WasmApp::new();
        // app.linker = <Linker<UserState>>::new();
        app.add_plugin(&mut leds)?
            .add_plugin(&mut time)?
            .add_plugin(&mut console)?;
        app.build()?;
        //     .build()?;

        // let mut sketch = SketchInstance::new(&mut app);
        // sketch.start();
        // frags.push([0; 1024]);
    }

    Ok(())
}
