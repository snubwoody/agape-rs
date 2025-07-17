# Changelog

All notable changes to agape will be documented in this file.

## 0.2.0

### Added

- Added `Resources` struct to share global resources.
- Repeat syntax, `hstack![widget;10]`, to `hstack!` and `vstack!` macros.
- Added `on_click` and `on_hover` gestures.
- Added `TextField` widget and text input.
- Added borders.
- `BoxStyle`, which contains common styling for all widgets.
- Added event systems, i.e. systems that run when specific events are emitted.

### Changed

- Systems now have a `&mut Resources` instead of the previous `&mut Context`.
- Most of the functionality, like layout and state, is now handled in systems.

### Removed

- Removed `Context` object, use `Resources` instead.
- (core) Removed deprecated `colors` module.

### Performance

- Use global font variable, instead of creating one each frame #77.
