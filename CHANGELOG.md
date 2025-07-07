# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0

### Added
- Added `Resources` struct to share global resources.
- Added repeat syntax (`hstack![widget;10]`) to `hstack!` and `vstack!` macros.

### Changed
- Systems now have a `&mut Resources` instead of the previous `&mut Context`.
- Most of the functionality, like layout and state, is now handled in systems. 

### Removed
- Removed `Context` object, use `Resources` instead.