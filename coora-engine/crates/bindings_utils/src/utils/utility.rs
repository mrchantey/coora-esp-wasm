use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse::Result, spanned::Spanned, *};

pub fn type_to_ident(ty: &Type) -> Result<Ident> {
	match ty {
		Type::Path(ty) => type_path_to_ident(ty),
		Type::Slice(ty) => type_to_ident(&ty.elem),
		_ => Err(Error::new(ty.span(), "Expected a path type")),
	}
}

pub fn type_path_to_ident(ty: &TypePath) -> Result<Ident> {
	let first = ty.path.segments.first();
	if let Some(first) = first {
		Ok(first.clone().ident)
	} else {
		Err(Error::new(ty.span(), "Expected a path segment"))
	}
}

pub fn pat_to_ident(pat: &PatType) -> Result<Ident> {
	if let Pat::Ident(ident) = &*pat.pat {
		Ok(ident.ident.clone())
	} else {
		Err(Error::new(pat.span(), "Expected an identifer"))
	}
}


pub fn fn_result_to_typed(item: &ReturnType) -> Result<TokenStream> {
	match &item {
		ReturnType::Type(_rarrow, rtype) => {
			if let Type::Path(rtype) = &**rtype {
				let ident = rtype.path.segments.first().unwrap().clone().ident;
				Ok(quote!(#ident))
			} else {
				// Err(Error::new(item.span(), "Hmmm whaaat."))
				Ok(quote!(()))
			}
		}
		ReturnType::Default => Ok(quote!(())),
	}
}

pub fn fn_arg_to_typed(item: &FnArg) -> Result<(PatIdent, PathSegment)> {
	if let FnArg::Typed(item) = item {
		let ty = if let Type::Path(rt) = &*item.ty {
			let first = rt.path.segments.first();
			if let Some(first) = first {
				Ok(first)
			} else {
				Err(Error::new(item.pat.span(), "Not sure how we got here.."))
			}
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
		// Ok(Ident::new("caching", item.span()))
		Err(Error::new(item.span(), "'self' is not a valid argument"))
	}
}
