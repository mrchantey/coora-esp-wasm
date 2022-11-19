use super::*;
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



pub fn create_plugin_implementation(plugin: &ItemTrait) -> Result<TokenStream> {
	let ident_plugin = &plugin.ident;
	let body = utils::BindingFuncDefinition::from_trait(&plugin)?;
	let body = body.iter().map(|f| {
		let input_t = TokenStream::from_iter(f.inputs.iter().map(|(_, b)| quote!(#b,)));
		let private_ident = Ident::new(format!("_{}", &f.sig.ident).as_str(), f.sig.span());
		let result_t = utils::fn_result_to_typed(&f.sig.output).unwrap();
		let public_ident = &f.sig.ident;
		(f,public_ident, private_ident, input_t, result_t)
	});
	let ident_plugin_str = ident_plugin.to_string();

	let mut definition_stream = TokenStream::from_iter(
		body.clone()
			.map(|(_,_, n_priv, t1, t2)| quote!(#n_priv:  wasmi::TypedFunc<(#t1),#t2>,)),
	);

	let mut exposure_stream = TokenStream::new();
	exposure_stream.append_all(body.clone().map(|(f,n_pub, n_priv, _, t2)| {
		let full_inputs = TokenStream::from_iter(f.inputs.iter().map(|(a,b)|quote!(#a:#b,)));
		let named_inputs = TokenStream::from_iter(f.inputs.iter().map(|(a,_)|quote!(#a,)));
		quote!(
			pub fn #n_pub(&mut self,#full_inputs) -> #t2 {
				self.#n_priv
					.call(&mut *self.store.lock().unwrap(), (#named_inputs))
					.unwrap()
			}
		)
	}));



	let mut creation_stream = TokenStream::new();
	creation_stream.append_all(body.clone().map(|(f,n_pub,n_priv,t1,t2)| {
		let n_pub_str = n_pub.to_string();

		quote!(#n_priv: instance
			.get_export(&mut *store_locked, #n_pub_str)
			.and_then(wasmi::Extern::into_func)
			.ok_or_else(|| panic!("could not find exported wasm function {}",#n_pub_str))
			.unwrap()
			.typed::<(#t1), #t2>(&mut *store_locked)
			.unwrap(),)
	}));

	let ident_struct = Ident::new(
		format!("{ident_plugin_str}Instance").as_str(),
		ident_plugin.span(),
	);

	Ok(quote! {
		pub struct #ident_struct<T> {
			store: coora_engine::SharedStore<T>,
			#definition_stream
		}

		impl<T> #ident_struct<T>{
			pub fn new(app: &mut WasmApp<T>) -> #ident_struct<T> {
				let instance = Some(app.instance).unwrap().unwrap();
					// instance.
				let store = std::sync::Arc::clone(&app.store);
				let mut store_locked = store.lock().unwrap();
				#ident_struct::<T> {
					store:std::sync::Arc::clone(&app.store),
					#creation_stream
				}
			}
			#exposure_stream
		}
	})
}
