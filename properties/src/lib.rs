use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,DeriveInput};

#[proc_macro_derive(Position)]
pub fn derive_position(input:TokenStream) -> TokenStream{
	// Parse the macro input
	let input = parse_macro_input!(input as DeriveInput);

	// Get the name of the struct
	let name = input.ident;

	// Get the generic parameters of the struct
	let generics = input.generics;
	let (impl_generics,ty_generics,_) = generics.split_for_impl();

	let expanded = quote!{
		impl #impl_generics crate::widgets::Drawable for #name #ty_generics {
			fn position(&mut self,x:i32,y:i32){
				dbg!("The macro was used");
				self.surface.x = x;
				self.surface.y = y;
			}

			fn get_position(&self) -> (i32,i32){
				(self.surface.x,self.surface.y)
			}

			fn size(&mut self,width:u32,height:u32) {
				self.surface.width = width as i32;
				self.surface.height = height as i32;
			}
		
			fn get_size(&self) -> (u32,u32) {
				(self.surface.width as u32,self.surface.height as u32)
			}
		}
	};

	TokenStream::from(expanded)
}

