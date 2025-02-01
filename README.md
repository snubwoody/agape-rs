# helium
An easy to use rust GUI library built using `wgpu`.

Helium is inspired by SwiftUI and Flutter and aims to be a fully featured rust gui library.

![](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

![example workflow](https://github.com/snubwoody/Helium/actions/workflows/rust.yml/badge.svg?branch=main)

## Quick start

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

# Support


|Platform|Status|
|--|:--:|
|Windows|✅|
|MacOS|➖|
|Linux|➖|
|Web|🚧|
|Android|🚧|
|IOS|🚧|

✅: Fully supported  
➖: Assumed but has not been tested  
🚧: Planned but not yet supported  
*For MacOS and Linux, all the libraries that `helium` uses, i.e `wgpu` and `winit`, are fully cross platform, however they have not been tested so support is not guaranteed. 
