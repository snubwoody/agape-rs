# helium
An easy to use rust GUI library built using `wgpu`.

Helium is inspired by SwiftUI and Flutter and aims to be a fully featured rust gui library.

![example workflow](https://github.com/snubwoody/Helium/actions/workflows/rust.yml/badge.svg?branch=main)

## Quick start

```rust
use helium::{App,widgets::Text,}

#[tokio::main]
async fn main() -> Result<(),helium::Error>{
	let text = Text::new("Hello world");

	let mut app = App::new();
	app.add_page(text)
	app.run().await
}
```

## Widgets

### Alignment

There are two axes for each widget, the main axis and the cross axis, for most widgets the main axis is in the x-direction with the cross axis.

<div align='center'>

![Axis Alignment Start](<docs/assets/Axis Alignment Start.svg>)
![Axis Alignment Center](<docs/assets/Axis Alignment Center.svg>)
![Axis Alignment End](<docs/assets/Axis Alignment End.svg>)
</div>

### HStack

The `HStack` is a `Widget` that arranges it's children horizontally.

![Illustration](<docs/assets/HStack Illustration.svg>)



## Support

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
