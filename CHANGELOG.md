# Changelog

All notable changes to agape will be documented in this file.

## [unreleased]

### Features

- Support for image rendering.
- Svg rendering using the `resvg` crate.

#### New widgets

- `Image`: Draw images to the screen
- `Svg`: Draw SVGs to the screen
- `Icon`: Idiomatic wrapper for `Svg`

### Changed

- Removed the `LayoutSolver` struct and renamed its single method to `solve_layout`.

### Removed

- Removed views, rendering is done through functions now.
- Removed widget iterators

## 0.2.0 - 2025-07-17

### Features

- Added `Resources` struct to share global resources
- Repeat syntax, `hstack![widget;10]`, to `hstack!` and `vstack!` macros
- Added `on_click` and `on_hover` gestures
- Added `TextField` widget and text input
- Added borders
- `BoxStyle`, which contains common styling for all widgets
- Added event systems, i.e. systems that run when specific events are emitted

### Changes

- Systems now have a `&mut Resources` instead of the previous `&mut Context`
- Most of the functionality, like layout and state, is now handled in systems

### Removed

- Removed `Context` object, use `Resources` instead
- (core) Removed deprecated `colors` module

### Performance

- Use global font variable, instead of creating one each frame [#77](https://github.com/snubwoody/agape-rs/pull/77)
