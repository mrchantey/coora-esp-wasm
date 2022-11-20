use super::*;
use crate::{func_args::parse_trait_funcs, utils};
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


pub fn create_trait(plugin: &ItemTrait) -> Result<TokenStream> {
	let ident_plugin = &plugin.ident;

	let ident_plugin_str = ident_plugin.to_string();
	let body = utils::BindingFuncDefinition::from_trait(&plugin)?;
	let body = body.iter();
	let funcs: Result<Vec<_>> = parse_trait_funcs(&plugin)?
		.iter()
		.map(|f| create_func(&ident_plugin_str, f))
		.collect();
	let funcs = TokenStream::from_iter(funcs?);

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
		// impl coora_engine::Shared for
		use wasmi::AsContext;//not sure this is the best way
		pub struct #ident_deorphaned<T>(std::sync::Arc<std::sync::Mutex<T>>);		//orphan rule https://doc.rust-lang.org/error_codes/E0210.html
		pub trait #ident_shared where Self: Sized{
			fn as_shared(self) -> #ident_deorphaned<Self> {  #ident_deorphaned::<Self>(std::sync::Arc::new(std::sync::Mutex::new(self))) }
		}
		impl<T> #ident_shared for T where T: #ident_plugin{}
		impl<T> coora_engine::Plugin for #ident_deorphaned<T> where
		T: #ident_plugin + std::marker::Send + 'static
		{
			fn bind(&mut self, app: &mut coora_engine::WasmApp)->anyhow::Result<()> {
				let store = std::sync::Arc::clone(&app.store);
				let mut store = store.lock().unwrap();
				//TODO if let some instance, throw
				#funcs
				Ok(())
			}
		}
	})
}
