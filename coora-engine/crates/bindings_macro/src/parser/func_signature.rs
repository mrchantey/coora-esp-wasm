use crate::*;
use proc_macro2::{Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use std::fmt::format;
use syn::{
	parse::{Parse, ParseStream, Result},
	parse_macro_input, Error,
};



pub struct FuncSignature {}


impl FuncSignature {
	pub fn parse<I>(iter: &mut I) -> Result<FuncSignature>
	where
		I: Iterator<Item = TokenTree>,
	{
		iter.next().assert_str("fn")?;
		// iter.next().as_lit_str()


		// let mut parameter_types: Vec<Type> = Vec::new();

		// let arg_types: ParseBuffer;
		// parenthesized!(arg_types in tokens);

		// while !arg_types.is_empty() {
		// 	let arg_type: Type = arg_types.parse()?;
		// 	parameter_types.push(arg_type);

		// 	if !arg_types.is_empty() {
		// 		let _comma: Token![,] = arg_types.parse()?;
		// 	}
		// }
		Ok(FuncSignature {})
	}
}
