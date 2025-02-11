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
# Widgets
## HStack
The `HStack` is a `Widget` that arranges it's children horizontally.

![Illustration](<docs/assets/HStack Illustration.svg>)

### Alignment

There are two axes for each widget, the main axis and the cross axis, for most widgets the main axis is in the x-direction with the cross axis

###### Start
![Start](<docs/assets/HStack Main-Axis-Alignment Start.svg>)

###### Center
![Center](<docs/assets/HStack Main-Axis-Alignment Center.svg>)

###### End
![End](<docs/assets/HStack Main-Axis-Alignment End.svg>)

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
