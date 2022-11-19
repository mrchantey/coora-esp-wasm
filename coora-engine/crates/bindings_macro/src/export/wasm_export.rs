use syn::{parse_macro_input,Result, parse::{Parse, ParseStream}};
use quote::quote;
use super::*;
pub struct WasmExport {
	pub out: proc_macro::TokenStream,
}

impl WasmExport {
	pub fn new(
		_attr: proc_macro::TokenStream,
		input: proc_macro::TokenStream,
	) -> proc_macro::TokenStream {
		parse_macro_input!(input as WasmExport).out
	}
}


impl Parse for WasmExport {
	fn parse(stream: ParseStream) -> Result<Self> {
		let plugin_trait = syn::ItemTrait::parse(stream)?;

		let export_impl = create_plugin_implementation(&plugin_trait)?;
		let out = quote!(
			#plugin_trait
			#export_impl
		).into();
		Ok(WasmExport { out })
	}
}
