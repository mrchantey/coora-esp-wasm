use coora_bindings_utils::import::create_trait;
use quote::quote;
use sweet::*;
use syn::*;

use crate::utility::{print_token_stream_in_main};

sweet! {

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
