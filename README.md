# Agape
An easy to use rust GUI library.

![example workflow](https://github.com/snubwoody/agape/actions/workflows/rust.yml/badge.svg?branch=main)

## Add to your project
```bash
cargo add agape
```

## Quick start

```rust
use agape::{App,widgets::Text};

fn main() -> Result<(),agape::Error>{
	let text = Text::new("Hello world");

	let mut app = App::new(text);
	app.run()
}
```


## Support

|Platform|Status|
|--|:-:|
|Windows|  ✅ |
|MacOS|  ✅|
|Linux|  ✅ |
|Android|  🚧 |
|IOS|  🚧 |

✅: Fully supported  
🚧: Planned but not yet supported  

## Roadmap
- [x] text rendering
- [ ] Grids
- [x] Row and column layouts
- [ ] Input handling
  - [ ] Hover events
  - [ ] Click events
  - [ ] Text input
- [ ] Scrolling

