// use anyhow::Ok;
use super::*;
use crate::utils::{pat_to_ident, type_path_to_ident, type_to_ident};
use quote::*;
use syn::*;


pub struct ParsedFuncArgs {
	pub together: Vec<(Ident, Ident)>,
	pub primitive: Vec<PrimitiveArg>,
	pub reference: Vec<ReferenceArg>,
}

pub fn parse_func_args(sig: &Signature) -> syn::parse::Result<ParsedFuncArgs> {
	let args_in = sig.inputs.iter().filter_map(|i| match i {
		FnArg::Typed(i) => Some(i),
		_ => None,
	});

	let together: Vec<_> = args_in
		.clone()
		.enumerate()
		.filter_map(|(index, arg)| match &*arg.ty {
			Type::Reference(pat) => {
				let name = pat_to_ident(&*arg).unwrap();
				let ty = type_to_ident(&*pat.elem).unwrap();
				let pointer_index = index;
				let name_ptr = Ident::new(format!("{name}_ptr").as_str(), name.span());
				let name_len = Ident::new(format!("{name}_len").as_str(), name.span());
				Some(Arg::Reference(ReferenceArg {
					name,
					ty,
					index,
					name_ptr,
					name_len,
				}))
			}
			Type::Path(pat) => {
				let name = pat_to_ident(&*arg).unwrap();
				let ty = type_path_to_ident(&*pat).unwrap();
				Some(Arg::Primitive(PrimitiveArg { name, ty, index }))
			}
			_ => None,
		})
		.collect();


	let reference: Vec<_> = together
		.iter()
		.filter_map(|arg| match arg {
			Arg::Reference(arg) => Some(arg.clone().to_owned()),
			_ => None,
		})
		.collect();
	let primitive: Vec<_> = together
		.iter()
		.filter_map(|arg| match arg {
			Arg::Primitive(arg) => Some(arg.clone().to_owned()),
			_ => None,
		})
		.collect();
	let together: Vec<_> = together
		.iter()
		.map(|arg| match arg {
			Arg::Reference(arg) => (arg.name.clone(), arg.ty.clone()),
			Arg::Primitive(arg) => (arg.name.clone(), arg.ty.clone()),
		})
		.collect();


	Ok(ParsedFuncArgs {
		together,
		primitive,
		reference,
	})
}
