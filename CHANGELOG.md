# CHANGELOG

## [1.2.1]

### Fixed

- Fix type in `#[optional]` macro Unnamed match from `&f_type` to `&#f_type`

## [1.2.0]

### Added

- Add checking for Named Structs in `#[derive(AutoGetters)]` macro
- Add checking for Named/Unnamed Structs in `#[optional]` macro
- Add documentation

### Removed

- Delete `helpers.rs` module

## [1.1.1]

### Added

- Add visibility to `#[optional]` macro

## [1.1.0]

### Added

- Add `#[optional]` procedural macro with attribute for structs

## [1.0.1]

### Fixed

- Fix mismatched type in `ouptut`: `#f_type` -> `&#f_type`

## [1.0.0]

### Added

- Add `auto_getters()` procedural macro for structs