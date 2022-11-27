use crate::utils_func::*;
// extern crate proc_macro;
use crate::utils::{pat_to_ident, type_path_to_ident, type_to_ident};
use proc_macro2::TokenStream;
use quote::*;
use syn::{*, spanned::Spanned};

pub fn create_func(plugin_name:&str,ParsedFunc { args, sig,index }: &ParsedFunc) -> parse::Result<TokenStream> {
	let ParsedFuncArgs {
		together,
		primitive,
		reference,
	} = &args;

	let contains_refs = reference.len() > 0;

	let func_args_ref = TokenStream::from_iter(reference.iter().map(|a| {
		let name_ptr = &a.name_ptr;
		let name_len = &a.name_len;
		quote!(#name_ptr: u32, #name_len:u32,)
	}));
	let func_args_pri = TokenStream::from_iter(primitive.iter().map(|a| {
		let name = &a.name;
		let ty = &a.ty;
		quote!(#name: #ty,)
	}));


	let ref_body = TokenStream::from_iter(reference.iter().map(
		|ReferenceArg {
		     name,
		     name_ptr,
		     name_len,
		     ty,
		     ..
		 }| {
			let ident_bytes = Ident::new(format!("{}_bytes", &name).as_str(), name.span());
			let bytes = quote!(let #ident_bytes = &memory.data(ctx)
				[#name_ptr as usize..(#name_ptr + #name_len) as usize];);
			if ty.to_string() == "str" {
				quote!(#bytes let #name = std::str::from_utf8(&#ident_bytes).unwrap();
				)
			} else {
				//only i32 for now,TODO calculate int length
				quote!(#bytes let #name = unsafe { 
					std::mem::transmute::<&[u8;#name_len],&[#ty]>(&#ident_bytes)};)
			}
		},
	));

	let func_name_str = sig.ident.to_string();
	let func_ident = Ident::new(func_name_str.as_str(),sig.span());
	let ident_mutex = Ident::new(format!("self{index}").as_str(), sig.span());
	let ident_mem = Ident::new(format!("mem{index}").as_str(), sig.span());
	// let full_inputs = TokenStream::from_iter(together.iter().map(|(a,b)|quote!(#a:#b,)));
	let named_inputs = TokenStream::from_iter(together.iter().map(|Arg{name,..}|quote!(#name,)));

	// quote!(

			// )
	let func_pre = if contains_refs {
		quote!(let #ident_mem = std::sync::Arc::clone(&app.memory);)
	} else {
		quote!()
	};
			
	let body_pre = if contains_refs {
		quote!(
			let memory = #ident_mem.lock().unwrap();
			let ctx = _caller.as_context();
		)
	} else {
		quote!()
	};
		
	let func = quote!(
		let #ident_mutex = std::sync::Arc::clone(&self.0);
		#func_pre

		app.linker
			.define(#plugin_name, #func_name_str, wasmi::Func::wrap(&mut *store,
				move |_caller:wasmi::Caller<coora_engine::UserState>,#func_args_ref #func_args_pri| { 
					#body_pre
					#ref_body
					#ident_mutex.lock().unwrap().#func_ident(#named_inputs) 
				})).unwrap();
	);

	Ok(func)
}
