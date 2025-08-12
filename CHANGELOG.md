# Changelog

All notable changes to agape will be documented in this file.

## [unreleased]

### Features

- Added support for emojis.
- Added support for images.
- Added support for svgs.

#### New widgets

- `Image`: Draw an image to the screen.
- `Svg`: Draws an svg to the screen.
- `Container`: Wrapper around another widget, useful for styling.

### Changed

- Removed the `LayoutSolver` struct and renamed its single method to `solve_layout`.

### Bug fixes

- Fixed incorrect text size

### Removed

- Removed views, rendering is done through functions now.
- Removed widget iterators
- Removed `Button` widget.
- Removed `hstack`, `vstack`, `input` examples.

## 0.2.0 - 2025-07-17

### Features

- Added `Resources` struct to share global resources
- Repeat syntax, `hstack![widget;10]`, to `hstack!` and `vstack!` macros
- Added `on_click` and `on_hover` gestures
- Added `TextField` widget and text input
- Added borders
- `BoxStyle`, which contains common styling for all widgets
- Added event systems, i.e. systems that run when specific events are emitted

### Changed

- Systems now have a `&mut Resources` instead of the previous `&mut Context`
- Most of the functionality, like layout and state, is now handled in systems

### Removed

- Removed `Context` object, use `Resources` instead
- (core) Removed deprecated `colors` module

### Performance

- Use global font variable, instead of creating one each frame [#77](https://github.com/snubwoody/agape-rs/pull/77)
