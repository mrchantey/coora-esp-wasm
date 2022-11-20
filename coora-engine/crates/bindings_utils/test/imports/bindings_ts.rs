use std::fmt::format;

use coora_bindings_utils::import::create_typescript_bindings_str;
use quote::quote;
use sweet::*;
use syn::*;

sweet! {

	it "works" {
		let stream = quote!(
			pub trait LedStrip {
				fn set_leds(&mut self, r: u32, g: u32, b: u32, w: u32);
				fn print(&mut self,val:&str);
				fn show(&mut self);
			}
		);
		let stream:ItemTrait = parse2(stream)?;

		let ts = create_typescript_bindings_str(&stream)?;
		// let ts = format!("{}",ts);
		println!("{ts}");
		// let stream = create_trait(&stream)?;
		// print_token_stream_in_main(quote!(#ts))?;
	}
}
