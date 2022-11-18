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


struct BindingFuncDefinition {
	index: usize,
	type_ident_func: Ident,
	type_ident_inputs: Ident,
	sig: Signature,
	inputs: Vec<(PatIdent, PathSegment)>,
}




fn parse_fn(index: usize, item: &TraitItem) -> Result<BindingFuncDefinition> {
	let type_ident_func = Ident::new(format!("T{}", index).as_str(), item.span());
	let type_ident_inputs = Ident::new(format!("TP{}", index).as_str(), item.span());
	if let TraitItem::Method(item) = item {
		let inputs: std::result::Result<Vec<_>, _> =
			item.sig.inputs.iter().map(|i| fn_arg_to_typed(i)).collect();
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



pub fn generate_bindings_definitions(plugin: &ItemTrait) -> Result<TokenStream> {
	let plugin_name = &plugin.ident;

	let plugin_name_str = plugin_name.to_string();
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
	let mut type_stream = TokenStream::new();

	type_stream.append_all(quote!(StoreT,));
	type_stream.append_all(body.clone().map(|f| {
		let i1 = &f.type_ident_func;
		let i2 = &f.type_ident_inputs;
		quote!(#i1,#i2,)
	}));

	let mut constraint_stream = TokenStream::new();
	constraint_stream.append_all(quote!(where));
	constraint_stream.append_all(body.clone().map(|f| {
		let ident_func = &f.type_ident_func;
		let ident_inputs = &f.type_ident_inputs;
		let mut inputs = TokenStream::new();
		inputs.append_all(f.inputs.iter().map(|i| {
			let t = &i.1;
			quote!(#t,)
		}));
		let output = &f.sig.output;
		// f.
		let result = fn_result_to_typed(output).unwrap();
		// output.
		// println!("\n\n{}\n\n", inputs.iter();
		quote!(#ident_func: 'static + Send + Sync 
		+ Fn(wasmi::Caller<StoreT>,#inputs)#output
		+ 'static
	 	+ wasmi::IntoFunc<StoreT,#ident_inputs, #result>
		,)
	}));

	let mut definition_stream = TokenStream::new();
	definition_stream.append_all(quote!(_marker: std::marker::PhantomData<StoreT>,));

	definition_stream.append_all(body.clone().map(|f| {
		let name = &f.sig.ident;
		let func_type = &f.type_ident_func;
		let inputs_type = &f.type_ident_inputs;
		let inputs_phantom = Ident::new((name.to_string() + "_inputs").as_str(),name.span());
		quote!(
			pub #name: #func_type,
			pub #inputs_phantom: #inputs_type,
			)
	}));

	let mut assign_stream = TokenStream::new();
	#[rustfmt::skip]
	assign_stream.append_all(body.clone().map(|f|{
		let func_name_str = f.sig.ident.to_string();
		let func_ident = Ident::new(func_name_str.as_str(),f.sig.span());
		quote!(builder.linker
		.define(#plugin_name_str, #func_name_str, wasmi::Func::wrap(&mut builder.store,def.#func_ident))
		.unwrap();)
	}));

	//names

	let def_name = Ident::new(
		(plugin_name_str.clone() + "Def").as_str(),
		plugin_name.span(),
	);
	let define_helper_name = Ident::new(
		(String::from("define") + plugin_name_str.as_str())
			.to_case(Case::Snake)
			.as_str(),
		plugin_name.span(),
	);
	let build_helper_name = Ident::new(
		(String::from("build") + plugin_name_str.as_str())
			.to_case(Case::Snake)
			.as_str(),
		plugin_name.span(),
	);
	Ok(quote! {
		// use wasmi::{Linker,Store,Caller};
		pub struct #def_name<#type_stream>#constraint_stream
		{
			#definition_stream
		}
		pub fn #define_helper_name<#type_stream>(val:#def_name<#type_stream>)->#def_name<#type_stream>#constraint_stream{
			val
		}
		pub fn #build_helper_name<#type_stream>(def:#def_name<#type_stream>,
			builder: &mut WasmInstanceBuilder<StoreT>)#constraint_stream{
			#assign_stream
		}
	})
}
