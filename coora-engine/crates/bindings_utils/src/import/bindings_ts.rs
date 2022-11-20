use std::fmt::format;

use crate::utils_func::{parse_trait_funcs, Arg, ParsedFunc, ReferenceArg};
use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::{
	parse::Result, spanned::Spanned, Error, FnArg, ItemTrait, LitStr, Pat, ReturnType, TraitItem,
	Type,
};


#[rustfmt::skip]
pub fn rust_method_to_ts(plugin_name: &str, func: &ParsedFunc) -> Result<String> {
	let return_type = get_return_type(func)?;
	let args_external = get_args_external(func);
	let args_internal = get_args_internal(func);
	let args_internal_names = get_args_internal_names(func);
	let ref_conversions = get_ref_conversions(func);

	let func_name = &func.sig.ident;
	let func_name_camel = func.sig.ident.to_string().to_case(Case::Camel);


	if func.args.reference.len() == 0 {
		return Ok(format!(
			"
//@ts-ignore external
@external(\'{plugin_name}\', \'{func_name}\')
export declare function {func_name_camel}({args_external}): {return_type};
",
		));
	}
		Ok(format!(
			"
//@ts-ignore external
@external(\'{plugin_name}\', \'{func_name}\')
declare function _{func_name_camel}({args_internal}): {return_type};			
export function {func_name_camel}({args_external}): void {{
	{ref_conversions}
	_{func_name_camel}({args_internal_names})
}}
	"
		))
}

const PREFIX: &str = "//AUTOGENERATED\n";
const SUFFIX: &str = r"";

pub fn create_typescript_bindings_str(plugin: &ItemTrait) -> Result<String> {
	let plugin_name = plugin.ident.to_string();
	let plugin_name = plugin_name.as_str();
	let funcs = parse_trait_funcs(plugin)?;
	let mut out = String::from(PREFIX);
	// let body = plugin.items.iter();
	for func in funcs.iter() {
		let ts_func = rust_method_to_ts(plugin_name, func)?;
		out.push_str(ts_func.as_str());
	}

	out.push_str(SUFFIX);
	Ok(out)
}
pub fn create_typescript_bindings(plugin: &ItemTrait) -> Result<LitStr> {
	let str = create_typescript_bindings_str(plugin)?;
	let out = LitStr::new(str.as_str(), Span::call_site());
	Ok(out)
}

//----------MINI FUNCS

fn get_return_type(func: &ParsedFunc) -> Result<String> {
	if let ReturnType::Type(_rarrow, rtype) = &func.sig.output {
		rust_type_to_ts(&**rtype)
	} else {
		Ok(format!("void"))
	}
}


#[rustfmt::skip]
fn get_args_internal(func:&ParsedFunc)->String{
	let mut args_internal: Vec<_> = func.args.reference.iter()
	.map(|f| {
		let ptr = f.name_ptr.to_string();
		let len = f.name_len.to_string();
		format!("{ptr}: u32, {len}: u32")
	}
).collect();
let mut next:Vec<_> = func.args.primitive.iter()
.map(|f| format!("{}:{}", f.name.to_string(), f.ty.to_string())).collect();
args_internal.append(&mut next);
args_internal.join(", ")
}
#[rustfmt::skip]
fn get_args_internal_names(func:&ParsedFunc)->String{
	let mut args_internal: Vec<_> = func.args.reference.iter()
	.map(|f| {
		let ptr = f.name_ptr.to_string();
		let len = f.name_len.to_string();
		format!("{ptr}, {len}")
	}
).collect();
let mut next:Vec<_> = func.args.primitive.iter()
.map(|f| format!("{}", f.name.to_string())).collect();
args_internal.append(&mut next);
args_internal.join(", ")
}

fn get_args_external(func: &ParsedFunc) -> String {
	let args_external: Vec<_> = func
		.args
		.together
		.iter()
		.map(|f| format!("{}:{}", f.name.to_string(), f.ty.to_string()))
		.collect();
	args_external.join(", ")
}

#[rustfmt::skip]
fn get_ref_conversions(func: &ParsedFunc) -> String {
	let reference_conversions:Vec<_> = func.args.reference.iter().enumerate()
		.map(|(i,ReferenceArg{name,name_ptr,name_len,..})|{
		let str_name = format!("{name_ptr}_str");
		format!("
	const {str_name} = String.UTF8.encode({name});
	let {name_ptr} = changetype<usize>(${str_name});
	let {name_len} = {str_name}.byteLength;
		")
	}).collect();
	reference_conversions.join("")
}



pub fn rust_pat_to_str(pat: &Pat) -> Result<String> {
	if let Pat::Ident(pat) = pat {
		Ok(pat.ident.to_string())
	} else {
		Err(Error::new(pat.span(), "Expected identifier"))
	}
}

pub fn rust_type_to_ts(rtype: &Type) -> Result<String> {
	if let Type::Path(rt) = rtype {
		Ok(rt.path.segments.first().unwrap().ident.to_string())
	} else {
		Err(Error::new(
			rtype.span(),
			"Currently only primitives are allowed",
		))
	}
}

/*
//@ts-ignore externam
@external("Serial", "println")
declare function _println(ptr: usize, len: i32): void;

export function println(str: string): void {
	const out = String.UTF8.encode(str)
	_println(changetype<usize>(out), out.byteLength)
}
*/
