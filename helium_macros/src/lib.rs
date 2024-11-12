use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn hex(item:TokenStream) -> TokenStream{
	let s = item.to_string().replace("\"","");
	println!("{}",&s);

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