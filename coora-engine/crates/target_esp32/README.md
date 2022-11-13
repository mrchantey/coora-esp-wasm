# ESP32 Target

## Troubleshooting

### Stack Overflow
`***ERROR*** A stack overflow in task main has been detected.`
- `C:\temp\esp32c3\riscv32imc-esp-espidf\release\build\esp-idf-sys-ee2e774c5bbb1a7b\out\build\C:/temp/.embuild/espressif/esp-idf/release-v4.4/components/esp_system\ubsan.c:294
- Try `cargo clean`
	- Not sure why this works, but it does
- Wasm Binary Stack size
	- To see whats happening here, convert your `.wasm` to a `.wat` and inspect the text
	- There may be a variable: `(global $__stack_pointer (mut i32) (i32.const 65536))`
	- Ensure that pointer size is <= `65536`