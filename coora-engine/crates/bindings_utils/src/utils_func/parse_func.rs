use super::*;
use crate::utils::{pat_to_ident, type_path_to_ident, type_to_ident};
use quote::*;
use syn::*;

pub struct ParsedFunc {
	pub index: usize,
	pub args: ParsedFuncArgs,
	pub sig: Signature,
}


pub fn parse_trait_funcs(plugin: &ItemTrait) -> Result<Vec<ParsedFunc>> {
	let funcs: Result<Vec<_>> = plugin
		.items
		.iter()
		.enumerate()
		.filter_map(|(i, f)| {
			if let TraitItem::Method(item) = f {
				Some(parse_func(&item.sig, i))
			} else {
				None
			}
		})
		.collect();
	Ok(funcs?)
}


pub fn parse_func(sig: &Signature, index: usize) -> Result<ParsedFunc> {
	let args = parse_func_args(sig)?;
	let func = ParsedFunc {
		index,
		args,
		sig: sig.clone(),
	};
	Ok(func)
}
