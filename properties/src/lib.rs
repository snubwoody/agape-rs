use std::process::id;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, token::Struct, DeriveInput, Expr, Member, Token};
use proc_macro::TokenStream;


#[proc_macro]
pub fn view(tokens:TokenStream) -> TokenStream {
	// Parse the macro input
	let input = parse_macro_input!(tokens as Expr);

	let tree = rustui::widgets::WidgetTree::new();

	match input {
		// Match against structs only
		syn::Expr::Struct(ref _struct) => {
			for (index,field) in _struct.fields.iter().enumerate(){
				match field.member {
					syn::Member::Named(ref ident) => {
						if ident == "child" {
						}
					}
					_ => {}
				}
			}
		}
		_ => {
			panic!("Wrong input")
		}
	}

	let expanded = quote!{
		/* rustui::app::view::View::new(
			#tree.build(#input)
		) */		
	};

	TokenStream::from(expanded)
}

