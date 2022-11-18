use crate::*;
// use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{
	parse::{Result},
	spanned::Spanned,
	Error, ItemTrait, PatIdent, PathSegment, Signature, TraitItem,
};


struct BindingFuncDefinition {
	_index: usize,
	sig: Signature,
	inputs: Vec<(PatIdent, PathSegment)>,
}

fn parse_fn(index: usize, item: &TraitItem) -> Result<BindingFuncDefinition> {
	if let TraitItem::Method(item) = item {
		let inputs: std::result::Result<Vec<_>, _> =
			item.sig.inputs.iter().map(|i| fn_arg_to_typed(i)).collect();
		let inputs = inputs?;
		// let inputs = vec![];

		Ok(BindingFuncDefinition {
			_index: index,
			inputs,
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
		
		let mut bind_stream = TokenStream::new();
		#[rustfmt::skip]
		bind_stream.append_all(body.clone().map(|f|{
			let func_name_str = f.sig.ident.to_string();
			let func_ident = Ident::new(format!("bind_{}",func_name_str).as_str(),f.sig.span());
			let result = fn_result_to_typed(&f.sig.output).unwrap();
		// let inputs = &f.sig.inputs;
				let mut inputs = TokenStream::new();
		inputs.append_all(f.inputs.iter().map(|i| {
			let t = &i.1;
			quote!(#t,)
		}));
		let output = &f.sig.output;
		let constraints = quote!(where 
		FuncT: 'static + Send + Sync 
		+ wasmi::IntoFunc<StoreT,P, #result>
		+ Fn(wasmi::Caller<StoreT>,#inputs)#output,
	);
		quote!(pub fn #func_ident<FuncT,P>(&self,
			builder: &mut coora_engine::WasmInstanceBuilder<StoreT>, 
			func:FuncT)->&Self #constraints {
				builder.linker.define(#plugin_name_str, #func_name_str, wasmi::Func::wrap(&mut builder.store,func)).unwrap();
				self
		})
	}));

	let def_name = Ident::new(
		(plugin_name_str.clone() + "Def").as_str(),
		plugin_name.span(),
	);
	Ok(quote! {
		pub struct #def_name<StoreT>{
			marker:core::marker::PhantomData<StoreT>
		}
		impl<StoreT> #def_name<StoreT>{
			pub fn new()->#def_name<StoreT>{
				#def_name::<StoreT>{
					marker:core::marker::PhantomData::<StoreT>
				}
			}
			#bind_stream
		}
	})
}
