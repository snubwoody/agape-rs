<h1 align="center">Agape</h1>
<p align="center">An easy to use rust GUI library.</p>
<div align="center">
  <img alt="Crates.io Version" src="https://img.shields.io/crates/v/agape">
  <img src="https://img.shields.io/docsrs/agape"/>
  <img src="https://img.shields.io/github/actions/workflow/status/snubwoody/agape-rs/rust.yml"/>
  <img alt="Crates.io License" src="https://img.shields.io/crates/l/agape">
  <img alt="Crates.io Size" src="https://img.shields.io/crates/size/agape">

</div>

## Goals

- Simple, expressive API
- No heavy use of macros
- Cross platform

## Overview

`agape` is a cross platform, CPU-rendered, GUI library. Internally it uses `tiny-skia` for rendering.

## Add to your project

```toml
[dependencies]
agape = "0.1.0"
```

## Quick start

```rust
use agape::{App, widgets::Text};

fn main() -> Result<(), agape::Error> {
    let text = Text::new("Hello world");

    let mut app = App::new(text);
    app.run()
}
```

## Support

| Platform | Status |
|----------|:------:|
| Windows  |   ✅    |
| MacOS    |   ✅    |
| Linux    |   ✅    |
| Android  |   🚧   |
| IOS      |   🚧   |

✅: Fully supported  
🚧: Planned

## Roadmap

- [x] text rendering
- [ ] Grids
- [x] Row and column layouts
- [ ] Input handling
    - [x] Hover events
    - [x] Click events
    - [ ] Text input
- [ ] Scrolling

## License

This project is dual-licensed under either

- Apache License, Version 2.0
- MIT License

at your choice.

