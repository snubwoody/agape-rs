use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput};

pub fn expand_widget(input: DeriveInput) -> proc_macro::TokenStream {
    let name = input.ident;
    let generics = input.generics;
    let child = get_children(&input.data).unwrap();
    quote! {
        impl agape::widgets::Widget for #name #generics {
            fn id(&self) -> GlobalId{
                self.id
            }

            fn children(&self) -> Vec<&dyn agape::widgets::Widget> {
                vec![&self.#child]
            }

            fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn agape::widgets::Widget)) {
                f(&mut self.#child);
                self.#child.traverse(f);
            }

            fn layout(&self, renderer: &mut agape::renderer::Renderer) -> Box<dyn agape::layout::Layout> {
                let child_layout = self.#child.layout(renderer);
                let mut layout = BlockLayout::new(child_layout);
                layout.id = self.id;
                Box::new(layout)
            }


            fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
                self.#child.render(renderer, layout);
            }
        }
    }
        .into()
}

fn get_children(data: &Data) -> Option<Ident> {
    let mut child = None;
    if let Data::Struct(data) = data {
        for field in &data.fields {
            if field.attrs.is_empty() {
                continue;
            }

            for attr in &field.attrs {
                if attr.meta.path().is_ident("child") {
                    child = field.ident.clone();
                }
            }

            dbg!(&field.attrs);
        }
    }
    child
}
