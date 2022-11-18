use crate::*;
use anyhow::anyhow;
use convert_case::{Case, Casing};
use proc_macro2::{Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::{quote, TokenStreamExt};
use std::fmt::format;
use syn::{
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
	spanned::Spanned,
	Error, FnArg, ItemTrait, LitStr, PatIdent, PathSegment, Signature, TraitItem,
};

pub struct BindingFuncDefinition {
	pub index: usize,
	pub type_ident_func: Ident,
	pub type_ident_inputs: Ident,
	pub sig: Signature,
	pub inputs: Vec<(PatIdent, PathSegment)>,
}


impl BindingFuncDefinition{
	
	pub fn parse(index: usize, item: &TraitItem) -> Result<BindingFuncDefinition> {
		let type_ident_func = Ident::new(format!("T{}", index).as_str(), item.span());
		let type_ident_inputs = Ident::new(format!("TP{}", index).as_str(), item.span());
		if let TraitItem::Method(item) = item {
			let inputs: std::result::Result<Vec<_>, _> = item
				.sig
				.inputs
				.iter()
				.map(|i| fn_arg_to_typed(i))
				.filter(|i| i.is_ok()) //skip invalid
				.collect();
			let inputs = inputs?;
			// let inputs = vec![];
	
			Ok(BindingFuncDefinition {
				index,
				inputs,
				type_ident_inputs,
				type_ident_func,
				sig: item.sig.clone(),
			})
		} else {
			Err(Error::new(
				item.span(),
				"Currently only functions are supported",
			))
		}
	}
	


}	

