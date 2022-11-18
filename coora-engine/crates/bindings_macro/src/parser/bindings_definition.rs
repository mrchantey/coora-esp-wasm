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


pub fn generate_bindings_definitions(plugin: &ItemTrait) -> Result<TokenStream> {
	let ident_plugin = &plugin.ident;

	let ident_plugin_str = ident_plugin.to_string();
	let body: std::result::Result<Vec<_>, _> = plugin
		.items
		.iter()
		.enumerate()
		.map(|(i, f)| BindingFuncDefinition::parse(i, f))
		.collect();
	let body = body?;
	let body = body.iter();

	let mut bind_stream = TokenStream::new();
	#[rustfmt::skip]
	bind_stream.append_all(body.clone().map(|f|{
		let func_name_str = f.sig.ident.to_string();
		let func_ident = Ident::new(func_name_str.as_str(),f.sig.span());
		let ident_mutex = Ident::new(format!("self{}",f.index).as_str(), f.sig.span());
		let full_inputs = TokenStream::from_iter(f.inputs.iter().map(|(a,b)|quote!(#a:#b,)));
		let named_inputs = TokenStream::from_iter(f.inputs.iter().map(|(a,_)|quote!(#a,)));
		// println!("\n\nHOHOH{}",full_inputs);
		// let full_inputs = ;
		// let full_inputs = &f.sig.inputs;
		quote!(
			let #ident_mutex = std::sync::Arc::clone(&self.0);
			builder.linker
				.define(#ident_plugin_str, #func_name_str, wasmi::Func::wrap(&mut builder.store,
				move |_:wasmi::Caller<StoreT>,#full_inputs| { #ident_mutex.lock().unwrap().#func_ident(#named_inputs) }))
				.unwrap();
		)
	}));

	//names
	let ident_shared = Ident::new(
		format!("Shared{ident_plugin_str}").as_str(),
		ident_plugin.span(),
	);
	let ident_deorphaned = Ident::new(
		format!("Deorphaned{ident_plugin_str}").as_str(),
		ident_plugin.span(),
	);
	let ident_shared = Ident::new(
		format!("Shared{ident_plugin_str}").as_str(),
		ident_plugin.span(),
	);

	Ok(quote! {
		// pub type #ident_plugin_shared = std::sync::Arc<std::sync::Mutex<#ident_plugin>>;
		pub struct #ident_deorphaned<T>(std::sync::Arc<std::sync::Mutex<T>>);		//orphan rule https://doc.rust-lang.org/error_codes/E0210.html
		// impl coora_engine::Shared for
		impl<T> coora_engine::Plugin for #ident_deorphaned<T> where
		T: #ident_plugin + std::marker::Send + 'static
		{
			fn bind<StoreT>(&mut self, builder: &mut coora_engine::WasmInstanceBuilder<StoreT>) {
				#bind_stream
			}
		}
		pub trait #ident_shared where Self: Sized{
			fn as_shared(self) -> #ident_deorphaned<Self> {  #ident_deorphaned::<Self>(std::sync::Arc::new(std::sync::Mutex::new(self))) }
		}
		impl<T> #ident_shared for T where T: #ident_plugin{}
	})
}

// let mut type_stream = TokenStream::new();

// type_stream.append_all(quote!(StoreT,));
// type_stream.append_all(body.clone().map(|f| {
// 	let i1 = &f.type_ident_func;
// 	let i2 = &f.type_ident_inputs;
// 	quote!(#i1,)
// }));

// let mut constraint_stream = TokenStream::new();
// constraint_stream.append_all(quote!(where));
// constraint_stream.append_all(body.clone().map(|f| {
// 	let ident_func = &f.type_ident_func;
// 	let ident_inputs = &f.type_ident_inputs;
// 	let mut inputs = TokenStream::new();
// 	inputs.append_all(f.inputs.iter().map(|i| {
// 		let t = &i.1;
// 		quote!(#t,)
// 	}));
// 	let output = &f.sig.output;
// 	// f.
// 	let result = fn_result_to_typed(output).unwrap();
// 	// output.
// 	// println!("\n\n{}\n\n", inputs.iter();
// 	quote!(#ident_func: 'static + Send + Sync
// 	+ Fn(wasmi::Caller<StoreT>,#inputs)#output
// 	+ 'static
// 	 + wasmi::IntoFunc<StoreT,#ident_inputs, #result>
// 	,)
// }));

// let mut definition_stream = TokenStream::new();
// definition_stream.append_all(quote!(_marker: std::marker::PhantomData<StoreT>,));

// definition_stream.append_all(body.clone().map(|f| {
// 	let name = &f.sig.ident;
// 	let type_name = &f.type_ident_func;
// 	quote!(pub #name: #type_name,)
// }));
