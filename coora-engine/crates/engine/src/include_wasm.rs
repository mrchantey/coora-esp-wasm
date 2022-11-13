#[macro_export]
macro_rules! include_wasm {
	($prefix:expr,$name:expr) => {
		include_bytes!(concat!(
			$prefix,
			"../../coora-app/dist/",
			$name,
			"/release.wasm"
		))
	};
}
#[macro_export]
macro_rules! include_rust_wasm {
	($prefix:expr,$name:expr) => {
		include_bytes!(concat!(
			$prefix,
			"../../coora-app-rs/target/wasm32-unknown-unknown/release/",
			$name,
			".wasm"
		))
	};
}
