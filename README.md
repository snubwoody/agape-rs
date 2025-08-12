<h1 align="center">Agape</h1>
<p align="center">An easy to use rust GUI library.</p>
<div align="center">
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/agape">
    <img src="https://img.shields.io/docsrs/agape"/>
    <img src="https://img.shields.io/github/actions/workflow/status/snubwoody/agape-rs/rust.yml"/>
    <img alt="Crates.io License" src="https://img.shields.io/crates/l/agape">
    <img alt="Crates.io Size" src="https://img.shields.io/crates/size/agape">
    <a href="https://codecov.io/gh/snubwoody/agape-rs" > 
        <img src="https://codecov.io/gh/snubwoody/agape-rs/graph/badge.svg?token=FNDNUZ7AGM"/> 
    </a>
</div>

## Goals

- Simple, expressive API
- No heavy use of macros
- Cross platform

## Overview

`agape` is a cross platform, CPU-rendered, GUI library.

## Add to your project

```toml
[dependencies]
agape = "0.2.0"
```

## Quick start

```rust
use agape::{App, widgets::*};

#[derive(Debug)]
struct TextBox {
    text: String
}

impl TextBox {
    pub fn new(text: &str) -> Self {
        Self { text: String::from(text) }
    }
}

impl View for TextBox {
    fn view() -> Box<dyn Widget> {
        Box::new(
            Text::new(&self.text)
        )
    }
}

fn main() -> Result<(), agape::Error> {
    let mut app = App::new(TextBox::new("Hi there"));
    app.run()
}
```

## Features

- [x] Text rendering
    - [x] Emoji support
    - [ ] RTL text
- [x] Image rendering
- [x] Svg rendering
- [ ] Cli

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

## License

This project is dual-licensed under either

- Apache License, Version 2.0
- MIT License

at your choice.

