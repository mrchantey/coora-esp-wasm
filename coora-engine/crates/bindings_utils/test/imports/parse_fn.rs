use crate::utility::print_token_stream_in_main;
use coora_bindings_utils::func_args::{create_import_func, parse_func};
use quote::quote;
use sweet::*;
use syn::*;

sweet! {
	test "transmute"{
		// let bytes:[u8;4] = [0,1,2,3];
		// let foo:&[u8] = &[0,1,2,3,4,5,6,7,8];

		// let bytes:&[u8;4] = &foo[0..4];
		// let bytes:[u8] = *bytes;

		// let a = unsafe { std::mem::transmute::<&[u8;4], &[i32;1]>(bytes) };
		// println!("answer is {:?}",a);
	}


	it "works" {
		let stream = quote!(fn foo(&self,a:&[i32],b:&str, c:i32)->i32;);
		let a:TraitItemMethod = parse2(stream)?;
		let func = parse_func(&a.sig, 0)?;
		// let args = parse_func_args(&a.sig)?;
		expect(func.args.primitive.len()).to_be(1)?;
		expect(func.args.reference.len()).to_be(2)?;

		let func_stream = create_import_func("MyMod",&func)?;
		print_token_stream_in_main(func_stream)?;
	}
}
