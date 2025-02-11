# helium
An easy to use rust GUI library built using `wgpu`.

Helium is inspired by SwiftUI and Flutter and aims to be a fully featured rust gui library.

![](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

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

![Axis Alignment Start](<docs/assets/Axis Alignment Start.svg>)
![Axis Alignment Center](<docs/assets/Axis Alignment Center.svg>)
![Axis Alignment End](<docs/assets/Axis Alignment End.svg>)

#### Main Axis

We can align sub-widgets on the main axis of supported widgets e.g. `HStack`, `VStack` and `Container`

```rust
use helium::{hstack,Text,crystal::AxisAlignment};

hstack!{
	Text::New("Hello"),
	Text::New("world"),
}
.main_axis_alignment(AxisAlignment::Center);
```

##### Start

Position sub-widget's at the start of the widget.

![Start](<docs/assets/Main-Axis-Alignment Start.svg>)

##### Center

Position sub-widget's at the center of the widget.

![Center](<docs/assets/Main-Axis-Alignment Center.svg>)

##### End

Position sub-widget's at the end of the widget.

![End](<docs/assets/Main-Axis-Alignment End.svg>)

#### Cross Axis

We can align sub-widgets on the cross axis of supported widgets e.g. `HStack`, `VStack` and `Container`


## HStack
The `HStack` is a `Widget` that arranges it's children horizontally.

![Illustration](<docs/assets/HStack Illustration.svg>)



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
