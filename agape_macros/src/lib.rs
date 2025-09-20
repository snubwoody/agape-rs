use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use std::{fs, path::Path};
use syn::{DeriveInput, parse_macro_input};

/// A macro for creating compile time verified hex colors.
#[proc_macro]
pub fn hex(item: TokenStream) -> TokenStream {
    let s = item.to_string().replace("\"", "");

    match agape_core::Color::hex(&s) {
        Ok(_) => quote! {agape::Color::hex(#s).unwrap()}.into(),
        Err(err) => {
            let message = format!("{err}");
            quote! {
                compile_error!(#message)
            }
            .into()
        }
    }
}

/// This macro does the very tedius job of defining a function for each icon in a
/// directory. The icon must be in svg format, all non-svg files in the directory will be ignored.
///
/// Files that start with reserved keywords will be prefixed with `_`.
/// eg `box.svg -> _box()`
#[proc_macro]
pub fn include_icons(dir: TokenStream) -> TokenStream {
    let dir_name = dir.to_string().replace("\"", "");
    let path = Path::new(&dir_name);

    let mut icons: Vec<proc_macro2::TokenStream> = vec![];

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                // TODO handle these unwraps
                let file = entry.unwrap();
                let entry_type = file.file_type().unwrap();

                if !entry_type.is_file() {
                    continue;
                }
                let path = file.path();

                // Skip non svg files
                if let Some(ext) = path.extension()
                    && ext != "svg"
                {
                    continue;
                }

                let raw_file_name = file.file_name();
                let mut file_name = raw_file_name
                    .to_str()
                    .unwrap()
                    .strip_suffix(".svg")
                    .unwrap()
                    .to_lowercase()
                    .replace(" ", "_") // Convert to snake case
                    .replace("-", "_");

                // Filter reserved keywords
                match file_name.as_str() {
                    "box" | "move" | "type" | "let" => file_name = format!("_{file_name}"),
                    _ => {}
                }

                let fn_name = proc_macro2::Ident::new(&file_name, Span::call_site());

                let svg_data = fs::read(path.as_path()).unwrap(); // TODO compile time check this
                let svg_data_literal = Literal::byte_string(&svg_data);

                icons.push(quote! {
                    pub fn #fn_name() -> crate::widgets::icon::Icon{
                        crate::widgets::icon::Icon::bytes(#svg_data_literal)
                    }
                });
            }
        }
        Err(err) => {
            let message = err.to_string();
            return quote! {
                compile_error!(#message)

            }
            .into();
        }
    }

    quote! {
        #(#icons)*
    }
    .into()
}

#[proc_macro_derive(Widget)]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    dbg!(&input.ident);

    let name = input.ident;
    quote! {
        impl agape::widgets::Widget for #name {
            fn id(&self) -> GlobalId{
                self.id
            }

            fn children(&self) -> Vec<&dyn agape::widgets::Widget> {
                vec![]
            }

            fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn agape::widgets::Widget)) {
                // f(&mut self.child);
                // self.child.traverse(f);
            }

            fn layout(&self, _: &mut agape::renderer::Renderer) -> Box<dyn agape::layout::Layout> {
                todo!();
            }


            fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
                // let layout = layout.get(self.id).unwrap();
                // let size = layout.size();
                // let position = layout.position();
                // let mut rect = Rect::new()
                //     .size(size.width, size.height)
                //     .position(position.x, position.y)
                //     .corner_radius(self.style.corner_radius)
                //     .color(self.style.background_color.clone());
                //
                // rect.border = self.style.border.clone();
                // renderer.draw_rect(rect);
                // self.child.render(renderer, layout);
            }
        }
    }
    .into()
}
