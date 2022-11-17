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
	Error, FnArg, ItemTrait, LitStr, PatIdent, PathSegment, Signature,
	TraitItem,
};


struct BindingFuncDefinition {
	index: usize,
	type_ident: Ident,
	sig: Signature,
	inputs: Vec<(PatIdent, PathSegment)>,
}




fn parse_fn(index: usize, item: &TraitItem) -> Result<BindingFuncDefinition> {
	let type_ident = Ident::new(format!("T{}", index).as_str(), item.span());
	if let TraitItem::Method(item) = item {
		let inputs: std::result::Result<Vec<_>, _> =
			item.sig.inputs.iter().map(|i| fn_arg_to_typed(i)).collect();
		let inputs = inputs?;
		// let inputs = vec![];

		Ok(BindingFuncDefinition {
			index,
			inputs,
			type_ident,
			sig: item.sig.clone(),
		})
	} else {
		Err(Error::new(
			item.span(),
			"Currently only functions are supported",
		))
	}
}



pub fn generate_bindings_definitions(
	plugin: &ItemTrait,
) -> Result<TokenStream> {
	let name = &plugin.ident;

	let name_str = name.to_string();
	let mut type_stream = TokenStream::new();
	let body: std::result::Result<Vec<_>, _> = plugin
		.items
		.iter()
		.enumerate()
		.map(|(i, f)| parse_fn(i, f))
		.collect();
	let body = body?;
	let body = body.iter();
	// body = body;
	// let a = body.map(|&(_, ident)| quote!(#ident));
	// let b = &body.map(|(_, ident)| quote!(#ident));

	type_stream.append_all(quote!(StoreT,));
	type_stream.append_all(body.clone().map(|f| {
		let i = &f.type_ident;
		quote!(#i,)
	}));

	let mut constraint_stream = TokenStream::new();
	constraint_stream.append_all(quote!(where));
	constraint_stream.append_all(body.clone().map(|f| {
		let ident = &f.type_ident;
		let mut inputs = TokenStream::new();
		inputs.append_all(f.inputs.iter().map(|i| {
			let t = &i.1;
			quote!(#t,)
		}));
		let output = &f.sig.output;
		// println!("\n\n{}\n\n", inputs.iter();
		quote!(#ident: FnMut(StoreT,#inputs)#output,)
	}));

	let mut definition_stream = TokenStream::new();
	definition_stream
		.append_all(quote!(_marker: std::marker::PhantomData<StoreT>,));


	definition_stream.append_all(body.clone().map(|f| {
		let name = &f.sig.ident;
		let type_name = &f.type_ident;
		quote!(pub #name: #type_name,)
	}));


	let def_name = Ident::new((name_str.clone() + "Def").as_str(), name.span());
	let helper_name = Ident::new(
		(String::from("define") + name_str.as_str())
			.to_case(Case::Snake)
			.as_str(),
		name.span(),
	);
	Ok(quote! {
		pub struct #def_name<#type_stream>#constraint_stream
		{
			#definition_stream
		}
		pub fn #helper_name<#type_stream>(val:#def_name<#type_stream>)->#def_name<#type_stream>#constraint_stream{
			val
		}
		impl<#type_stream> #def_name<#type_stream> #constraint_stream{
			pub fn new(){}

		}
	})
}
