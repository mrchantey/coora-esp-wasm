#![allow(dead_code, unused_imports, unused_mut, unused_variables, unused_parens)]
// use crate::coora_plugin::
use proc_macro::TokenStream;
mod parser;
use parser::*;

#[proc_macro_attribute]
pub fn coora_plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
	CooraPlugin::new(_attr, item)
}
