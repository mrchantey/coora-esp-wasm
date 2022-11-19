use super::*;
use crate::utils;
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


pub fn create_plugin_implementation(plugin: &ItemTrait) -> Result<TokenStream> {
	let ident_plugin = &plugin.ident;

	let ident_plugin_str = ident_plugin.to_string();
	let body = utils::BindingFuncDefinition::from_trait(&plugin)?;
	let body = body.iter();

	let mut bind_stream = TokenStream::new();
	#[rustfmt::skip]
	bind_stream.append_all(body.clone().map(|f|{
		let func_name_str = f.sig.ident.to_string();
		let func_ident = Ident::new(func_name_str.as_str(),f.sig.span());
		let ident_mutex = Ident::new(format!("self{}",f.index).as_str(), f.sig.span());
		let full_inputs = TokenStream::from_iter(f.inputs.iter().map(|(a,b)|quote!(#a:#b,)));
		let named_inputs = TokenStream::from_iter(f.inputs.iter().map(|(a,_)|quote!(#a,)));

		quote!(
			let #ident_mutex = std::sync::Arc::clone(&self.0);
			app.linker
				.define(#ident_plugin_str, #func_name_str, wasmi::Func::wrap(&mut *store,
				move |_:wasmi::Caller<coora_engine::UserState>,#full_inputs| { #ident_mutex.lock().unwrap().#func_ident(#named_inputs) }))
				.unwrap();
		)
	}));

	let ident_shared = Ident::new(
		format!("Shared{ident_plugin_str}").as_str(),
		ident_plugin.span(),
	);
	let ident_deorphaned = Ident::new(
		format!("Deorphaned{ident_plugin_str}").as_str(),
		ident_plugin.span(),
	);

	Ok(quote! {
		// pub type #ident_plugin_shared = std::sync::Arc<std::sync::Mutex<#ident_plugin>>;
		pub struct #ident_deorphaned<T>(std::sync::Arc<std::sync::Mutex<T>>);		//orphan rule https://doc.rust-lang.org/error_codes/E0210.html
		// impl coora_engine::Shared for
		impl<T> coora_engine::Plugin for #ident_deorphaned<T> where
		T: #ident_plugin + std::marker::Send + 'static
		{
			fn bind(&mut self, app: &mut coora_engine::WasmApp)->anyhow::Result<()> {
				let store = std::sync::Arc::clone(&app.store);
				let mut store = store.lock().unwrap();
				//TODO if let some instance, throw
				#bind_stream
				Ok(())
			}
		}
		pub trait #ident_shared where Self: Sized{
			fn as_shared(self) -> #ident_deorphaned<Self> {  #ident_deorphaned::<Self>(std::sync::Arc::new(std::sync::Mutex::new(self))) }
		}
		impl<T> #ident_shared for T where T: #ident_plugin{}
	})
}
