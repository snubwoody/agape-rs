# Ruby

General purpose native 2D renderer built using `wgpu`

## Getting started
```rust
use ruby::{Color, Rect};


#[tokio::main]
async fn main() -> ruby::Result<()> {
    let app = ruby::App::new()?;
    
    app.run(move |r|{
        let rect = Rect::new(500.0, 500.0).color(Color::rgb(25, 233, 102));
        r.draw_rect(rect);
    }).await?;
    
    Ok(())
}

```

## Features

- ✅ Basic shapes 
- ✅ Image rendering
- ✅ Text rendering
- ✅ GPU accelerated
  
## Primitives

### Rect

A rectangular shape
