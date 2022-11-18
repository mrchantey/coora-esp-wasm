use crate::*;
use quote::quote;
use syn::{
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
};


pub struct CooraPlugin {
	pub out: proc_macro::TokenStream,
}

impl CooraPlugin {
	pub fn new(
		_attr: proc_macro::TokenStream,
		input: proc_macro::TokenStream,
	) -> proc_macro::TokenStream {
		parse_macro_input!(input as CooraPlugin).out
	}
}


impl Parse for CooraPlugin {
	fn parse(stream: ParseStream) -> Result<Self> {
		let plugin_trait = syn::ItemTrait::parse(stream)?;
		// let a = plugin_trait.items.iter().next().unwrap();

		let name = &plugin_trait.ident;
		let name_str = name.to_string();

		let typescript_bindings = generate_typescript_bindings(&plugin_trait)?;
		let rust_bindings = generate_rust_bindings(&plugin_trait);
		let bindings_definitions = generate_bindings_definitions(&plugin_trait)?;

		let out = quote! {
				#plugin_trait
				//TODO better flag
				#[cfg(any(target_os="windows",target_os="linux",target_os="macos"))]
				inventory::submit!(coora_bindings::CooraPluginBindings {
					name: #name_str,
					typescript_bindings: #typescript_bindings,
					rust_bindings: #rust_bindings,
				});
				#bindings_definitions
		}
		.into();

		Ok(CooraPlugin { out })
	}
}
