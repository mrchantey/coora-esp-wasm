// use anyhow::Ok;
use super::*;
use crate::utils::{pat_to_ident, type_path_to_ident, type_to_ident};
use quote::*;
use syn::*;


pub struct ParsedFuncArgs {
	pub together: Vec<Arg>,
	pub primitive: Vec<PrimitiveArg>,
	pub reference: Vec<ReferenceArg>,
}

pub fn parse_func_args(sig: &Signature) -> syn::parse::Result<ParsedFuncArgs> {
	let together: Vec<_> = sig
		.inputs
		.iter()
		.enumerate()
		.filter_map(|(index, a)| match a {
			FnArg::Typed(a) => {
				let name = pat_to_ident(&*a.pat).unwrap();
				let ty = type_to_ident(&*a.ty).unwrap();
				let i = Arg {
					index,
					name,
					ty,
					pat_ty: a.clone(),
				};
				Some(i)
			}
			_ => None,
		})
		.collect();

	let reference: Vec<_> = together
		.iter()
		.filter_map(|arg| match &*arg.pat_ty.ty {
			Type::Reference(pat) => {
				let name = arg.name.clone();
				let ty = arg.ty.clone();
				let name_ptr = Ident::new(format!("{name}_ptr").as_str(), name.span());
				let name_len = Ident::new(format!("{name}_len").as_str(), name.span());
				Some(ReferenceArg {
					name,
					index: arg.index,
					ty,
					name_ptr,
					name_len,
				})
			}
			_ => None,
		})
		.collect();
	let primitive: Vec<_> = together
		.iter()
		.filter_map(|arg| match &*arg.pat_ty.ty {
			Type::Path(pat) => Some(PrimitiveArg {
				name: arg.name.clone(),
				ty: arg.ty.clone(),
				index: arg.index,
			}),
			_ => None,
		})
		.collect();


	Ok(ParsedFuncArgs {
		together,
		primitive,
		reference,
	})
}
