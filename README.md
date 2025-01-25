# Helium-rs
Rust UI library built using `wgpu`.

![](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

![example workflow](https://github.com/snubwoody/Helium/actions/workflows/rust.yml/badge.svg)

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


