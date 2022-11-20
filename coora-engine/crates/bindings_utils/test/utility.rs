use anyhow::{Ok, Result};
use prettyplease::*;
use proc_macro2::TokenStream;
use quote::*;
use syn::*;

pub fn print_token_stream(stream: TokenStream) -> Result<()> {
	let a: Item = parse2(stream)?;
	let out = unparse(&syn::File {
		attrs: vec![],
		items: vec![a],
		shebang: None,
	});
	println!("{}", out);
	Ok(())
}
pub fn print_token_stream_in_main(stream: TokenStream) -> Result<()> {
	print_token_stream(quote!(fn main(){#stream}))
}
