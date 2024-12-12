use proc_macro::TokenStream;
use quote::quote;

/// A hex color, this macro panics if an invalid hex color is used. Three letter
/// hex colors are not supported neither are hex colors with an alpha attribute.
#[proc_macro]
pub fn hex(item:TokenStream) -> TokenStream{
	let s = item.to_string().replace("\"","");

	match helium_core::color::Color::hex_to_rgba(&s) {
		Ok(_) => {
			return quote! {helium::Color::Hex(String::from(#s))}.into()
		},
		Err(err) => {
			println!("{}",err);
			return quote! {
				compile_error!(#err)
			}.into()
		},
	}
	
	
}