use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DeriveInput};

pub fn expand_widget(input: DeriveInput) -> proc_macro::TokenStream {
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let child = get_children(&input.data).unwrap();
    let impl_update = impl_update(&input.attrs);
    let impl_children = impl_children(&child);
    quote! {
        impl #impl_generics agape::widgets::Widget for #name #ty_generics #where_clause {
            fn id(&self) -> GlobalId{
                self.id
            }

            #impl_update

            #impl_children

            fn layout(&self, renderer: &mut agape::renderer::Renderer) -> Box<dyn agape::layout::Layout> {
                let child_layout = self.#child.layout(renderer);
                let mut layout = agape::layout::BlockLayout::new(child_layout);
                layout.id = self.id;
                Box::new(layout)
            }


            fn render(
                &self,
                renderer: &mut agape::renderer::Renderer,
                layout: &dyn agape::layout::Layout
            ) {
                self.#child.render(renderer, layout);
            }
        }
    }
        .into()
}

/// Implement `childen` and `traverse` methods.
fn impl_children(child: &Ident) -> TokenStream {
    quote! {
        fn children(&self) -> Vec<&dyn agape::widgets::Widget> {
            vec![&self.#child]
        }

        fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn agape::widgets::Widget)) {
            f(&mut self.#child);
            self.#child.traverse(f);
        }
    }
}

fn impl_update(attrs: &[Attribute]) -> TokenStream {
    let mut interactive = false;
    for attr in attrs {
        if attr.meta.path().is_ident("interactive") {
            interactive = true;
        }
    }

    if !interactive {
        return TokenStream::new();
    }

    quote! {
        fn tick(&mut self, messages: &mut agape::message::MessageQueue){
            self.update(messages);
        }
    }
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
        }
    }
    child
}
