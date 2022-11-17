use anyhow::anyhow;
use proc_macro2::{Group, Ident, Literal, Punct, Span, TokenTree};
use quote::quote;
use syn::{
	parse::{Parse, ParseStream, Result},
	parse_macro_input, Error,
};

use extend::ext;

#[ext]
pub impl Option<TokenTree> {
	// fn as_existing(self) -> Result<TokenTree> {
	// 	self.ok_or(Error::new(Span::call_site(), "expected a token here"))
	// }
	fn as_existing(self) -> Result<TokenTree> {
		if let Some(token) = self {
			return Ok(token);
		} else {
			Err(Error::new(Span::call_site(), "expected a token here"))
		}
	}

	fn assert_exists(&self) -> Result<()> {
		if let Some(token) = self {
			return Ok(());
		} else {
			Err(Error::new(Span::call_site(), "expected a token here"))
		}
	}
	fn assert_str(self, str: &str) -> Result<()> {
		let token = self.as_existing()?;

		// let token = self.ok_or();
		let token_str = token.to_string();
		if token_str == str {
			Ok(())
		} else {
			Err(Error::new(
				token.span(),
				format!("expected {str}, found {token_str}"),
			))
		}
	}

	// fn assert_literal(&self)->Result<()>{
	// 	if
	// }
	fn as_ident(self) -> Result<Ident> {
		let token = self.as_existing()?;
		if let TokenTree::Ident(token) = token {
			Ok(token)
		} else {
			Err(Error::new(
				token.span(),
				format!("expected identifier, found {token}"),
			))
		}
	}
	fn as_lit_str(self) -> Result<Literal> {
		let token = self.as_existing()?;
		if let TokenTree::Literal(token) = token {
			// Literal::str
			// token.inner
			// syn::LitStr
			Ok(token)
		} else {
			Err(Error::new(
				token.span(),
				format!("expected literal, found {token}"),
			))
		}
	}
	fn as_punct(self) -> Result<Punct> {
		let token = self.as_existing()?;
		if let TokenTree::Punct(token) = token {
			Ok(token)
		} else {
			Err(Error::new(
				token.span(),
				format!("expected punctuation, found {token}"),
			))
		}
	}
	fn as_group(self) -> Result<Group> {
		let token = self.as_existing()?;
		if let TokenTree::Group(token) = token {
			Ok(token)
		} else {
			Err(Error::new(
				token.span(),
				format!("expected group, found {token}"),
			))
		}
	}
}
