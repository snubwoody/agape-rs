# Helium-rs

Rust UI library built using `wgpu`.

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


