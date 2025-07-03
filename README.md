# agape
An easy to use rust GUI library.

![example workflow](https://github.com/snubwoody/agape/actions/workflows/rust.yml/badge.svg?branch=main)

## Quick start

```rust
use agape::{App,widgets::Text};

fn main() -> Result<(),agape::Error>{
	let text = Text::new("Hello world");

	let mut app = App::new(text);
	app.run()
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
|--|:-:|
|Windows|  ✅ |
|MacOS|  ✅|
|Linux|  ✅ |
|Web|  🚧 |
|Android|  🚧 |
|IOS|  🚧 |

✅: Fully supported  
🚧: Planned but not yet supported  

## Roadmap
- [x] text rendering
- [ ] Grids
- [x] Row and column layouts
- [ ] Input handling
  - [x] Hover events
  - [ ] Click events
  - [ ] Text input
- [ ] Scrolling

## Names
- cracoa
- coco
- agape
- philia
