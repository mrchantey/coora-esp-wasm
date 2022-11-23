use anyhow::Result;
use coora_engine::*;
use coora_target_esp32::*;
/*
better if you throw out whole thing?


*/
fn main() -> Result<()> {
    let mut esp32 = take_esp32_imports()?;
    // let mut app = WasmApp::new();

    for i in 0..100 {
        println!("running {i}");
        // .build_with_wasm(&include_wasm!("../../", "hello_world")[..]);
        let mut app = WasmApp::new();
        // app.linker = <Linker<UserState>>::new();
        app.add_plugin(&mut esp32)?
            .add_plugin(&mut StdImports)?
            .build()?;

        // let mut sketch = SketchInstance::new(&mut app);
        // sketch.start();
        // frags.push([0; 1024]);
    }

    Ok(())
}
