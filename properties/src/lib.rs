use quote::{quote};
use syn::{
	parse_macro_input, 
	Expr, ItemFn, 
};
use proc_macro::TokenStream;


#[proc_macro]
pub fn graph(tokens:TokenStream) -> TokenStream {
	// Parse the macro input
	let input = parse_macro_input!(tokens as Expr);

	let tree = rustui::widgets::WidgetTree::new();
	dbg!(input);
	let expanded = quote!{
		/* rustui::app::view::View::new(
			#tree.build(#input)
		) */		
	};

	TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn page(attribute:TokenStream,tokens:TokenStream) -> TokenStream{
	let input = parse_macro_input!(tokens as ItemFn);	

	dbg!(input.block.stmts);
	match input.block.stmts{
		_ =>{}
	}

	let expanded = quote! {
		fn new_app(){}
	};

	TokenStream::from(expanded)
}

