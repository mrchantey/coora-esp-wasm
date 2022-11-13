# ESP32 Target

## Troubleshooting
### Object Path Max
`CMAKE_OBJECT_PATH_MAX`
- Path too long? you may be able to get away with just using powershell instead of cygwin
- Otherwise you need to reduce path length, DO NOT specify a target dir other than default, it will randomly 'stack overflow' at RUNTIME

### Stack Overflow

`***ERROR*** A stack overflow in task main has been detected.`
- bla bla `release-v4.4/components/esp_system\ubsan.c:294
- Are you specifying a target directory? if so, dont do that because it doesnt work in really weird ways, i think its a linker problem.
- Try `cargo clean`
- Wasm Binary Stack size
	- To see whats happening here, convert your `.wasm` to a `.wat` and inspect the text
	- There may be a variable: `(global $__stack_pointer (mut i32) (i32.const 65536))`
	- Ensure that pointer size is <= `65536`