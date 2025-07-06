# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added `Resources` struct to share global resources
- Added `EventQueue`
- Added event systems i.e. systems that run when a specific event is emmitted

### Changed
- Systems now have a `&mut Resources` parameter instead of the previous `&mut Context`
