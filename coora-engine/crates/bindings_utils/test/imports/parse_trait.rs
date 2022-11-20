use coora_bindings_utils::import::create_trait;
use quote::quote;
use sweet::*;
use syn::*;

use crate::utility::{print_token_stream_in_main};

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
		let stream = quote!(
			pub trait LedStrip {
				fn set_leds(&mut self, r: u32, g: u32, b: u32, w: u32);
				fn show(&mut self);
			}
		);
		let stream:ItemTrait = parse2(stream)?;
		let stream = create_trait(&stream)?;
		print_token_stream_in_main(stream)?;
	}
}
