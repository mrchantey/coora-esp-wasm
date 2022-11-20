#![allow(dead_code, unused_imports, unused_mut, unused_variables, unused_parens)]
use proc_macro::TokenStream;
// mod export;
// mod import;
use coora_bindings_utils::{export, import};
// mod utils;

#[proc_macro_attribute]
pub fn coora_import(_attr: TokenStream, item: TokenStream) -> TokenStream {
	import::WasmImport::new(_attr, item)
}
#[proc_macro_attribute]
pub fn coora_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
	export::WasmExport::new(_attr, item)
}
