## [0.5.0] - 2026-01-17

### ⚠ BREAKING CHANGES

- **Constructors**: `ReCase::new()` now exclusively accepts string slices (`&str`). Passing an owned `String` is no longer supported (use `&str` or `.as_str()`).
- **Methods**: Removed the `original_case()` method.
- **Deprecations**: Removed all previously deprecated constructors.

### Performance

- **Zero-Allocation Logic**: Completely rewrote the core logic to prioritize Zero-Allocation splitting and Single-Allocation result building. The library now allocates memory exactly once (for the result string), making it significantly faster and more memory-efficient than previous versions.

### Changed

- **Input Handling**: `ReCase` struct now holds a reference (`&'a str`) instead of an owned `String`, reducing unnecessary cloning during initialization.

---

## [0.4.0] - 2026-01-07

### Added

- **Casing Trait**: Introduced a new extension trait implemented for `str` and `String`, allowing direct method calls like `"text".to_snake_case()`.

### Changed

- **Optimized Memory Allocation**: Refactored internal casing logic to use a single-buffer allocation strategy, significantly reducing heap allocations.
- **Improved Panic Safety**: Removed panics when receiving empty strings.
- **Naming Convention**: Standardized trait methods with the `to_` prefix (e.g., `to_camel_case`).
