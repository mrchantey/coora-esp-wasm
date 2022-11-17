use std::fmt::format;

use crate::*;
use anyhow::anyhow;
use proc_macro2::{Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::{quote, TokenStreamExt};
use syn::{
	parse::{Parse, ParseStream, Result},
	parse_macro_input, Error, ItemTrait,
};



pub fn generate_rust_bindings(plugin: &ItemTrait) -> TokenStream {
	// let funcs = inner.stream();
	// let name_str = name.
	let name = &plugin.ident;

	let name_str = name.to_string();
	let body = plugin.items.iter();
	let mut stream = TokenStream::new();
	stream.append_all(body);
	// pub mod #name{
	// pub mod #name{
	// }
	quote! {
		#[link(wasm_import_module = #name_str)]
		extern "C" {
			#stream
			}
	}
}
