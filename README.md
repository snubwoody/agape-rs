# helium
Rust UI library built using `wgpu`.
Helium is inspired by SwiftUI and Flutter and aims to be a fully featured rust gui library.

![](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

![example workflow](https://github.com/snubwoody/Helium/actions/workflows/rust.yml/badge.svg?branch=main)

## Getting started

```rust
use helium::{
	widget::Text,
	app::App,
	page::Page
}

fn main(){
	let page = Page::new(Text::new("Hello world"));

	App::new()
		.add_page(page)
		.run()
		.unwrap()
}
```


