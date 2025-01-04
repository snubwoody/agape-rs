use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use std::{fs, path::Path};

/// A hex color, this macro panics if an invalid hex color is used. Three letter
/// hex colors are not supported neither are hex colors with an alpha attribute.
#[proc_macro]
pub fn hex(item: TokenStream) -> TokenStream {
    let s = item.to_string().replace("\"", "");

    match helium_core::color::Color::hex_to_rgba(&s) {
        Ok(_) => return quote! {helium::Color::Hex(#s)}.into(),
        Err(err) => {
            return quote! {
                compile_error!(#err)
            }
            .into()
        }
    }
}

/// This macro does the very tedius job of defining a function for each icon in a
/// directory. The icon must be in svg format, all non-svg files in the directory will be ignored.
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
                // TODO handle these unwrraps
                let file = entry.unwrap();
                let entry_type = file.file_type().unwrap();

                if !entry_type.is_file() {
                    continue;
                }
                let path = file.path();

                // Skip non svg files
                if let Some(ext) = path.extension() {
                    if ext != "svg" {
                        continue;
                    }
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
                    "box" | "move" | "type" | "let" => file_name = format!("_{}", file_name),
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

    return quote! {
        #(#icons)*
    }
    .into();
}
