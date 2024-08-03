use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,DeriveInput};

#[proc_macro_derive(Position)]
pub fn derive_position(input:TokenStream) -> TokenStream{
	// Parse the macro input
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;

	let expanded = quote!{
		impl #name {
			fn position(&mut self,x:i32,y:i32){
				self.surface.x = x;
				self.surface.y = y;
			}

			fn get_position(&mut self) -> (i32,i32){
				(self.surface.x,self.surface.y)
			}
		}
	};

	TokenStream::from(expanded)
}

