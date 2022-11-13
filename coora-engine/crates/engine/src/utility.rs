#[macro_export]
macro_rules! include_wasm {
	($prefix:expr,$name:expr) => {
		include_bytes!(concat!($prefix,"../../coora-app/dist/",$name,"/release.wasm"))
		// include_bytes!(concat!($prefix,$name,"/target/wasm32-unknown-unknown/release/",$name,".wasm"))
	};
}
#[macro_export]
macro_rules! include_rust_wasm {
	($prefix:expr) => {
		include_bytes!(concat!($prefix,"../../coora-app/rust_example/target/wasm32-unknown-unknown/release/rust_example.wasm"))
	};
}
