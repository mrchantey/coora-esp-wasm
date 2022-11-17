use std::fmt::format;

use crate::*;
// use anyhow::anyhow;
use proc_macro2::{Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use syn::{
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
	spanned::Spanned,
	token::{RArrow, Trait},
	Error, FnArg, ItemTrait, LitStr, Pat, PatIdent, PathSegment, ReturnType,
	Token, TraitItem, Type,
};

pub fn fn_arg_to_typed(item: &FnArg) -> Result<(PatIdent, PathSegment)> {
	if let FnArg::Typed(item) = item {
		// item.attrs[0].
		let ty = if let Type::Path(rt) = &*item.ty {
			let first = rt.path.segments.first();
			if let Some(first) = first {
				Ok(first)
			} else {
				Err(Error::new(item.pat.span(), "Not sure how we got here.."))
			}
		// println!("\n\n{:?}\n\n", first);
		} else {
			Err(Error::new(
				item.pat.span(),
				"Currently only primitives are allowed, ie u32",
			))
		}?;
		let ident = if let Pat::Ident(ident) = &*item.pat {
			Ok(ident)
		} else {
			Err(Error::new(
				item.pat.span(),
				"Expected identifier, ie my_var",
			))
		}?;
		Ok((ident.clone(), ty.clone()))
	} else {
		Err(Error::new(item.span(), "'self' is not a valid argument"))
	}
}
