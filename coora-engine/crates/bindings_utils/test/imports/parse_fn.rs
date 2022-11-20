use crate::utility::print_token_stream_in_main;
use coora_bindings_utils::{func_args::parse_func, import::create_func};
use quote::quote;
use sweet::*;
use syn::*;

sweet! {
		let stream = quote!(fn foo(&self,a:&[i32],b:&str, c:i32)->i32;);


	it "works" {
		let a:TraitItemMethod = parse2(stream)?;
		let func = parse_func(&a.sig, 0)?;
		// let args = parse_func_args(&a.sig)?;
		expect(func.args.primitive.len()).to_be(1)?;
		expect(func.args.reference.len()).to_be(2)?;

		let func_stream = create_func("MyMod",&func)?;
		print_token_stream_in_main(func_stream)?;
	}
}
