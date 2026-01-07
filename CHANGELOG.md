## [0.4.0] - 2026-01-07

### Added

-   **Casing Trait**: Introduced a new extension trait implemented for `str` and `String`, allowing direct method calls like `"text".to_snake_case()`.

### Changed

-   **Optimized Memory Allocation**: Refactored internal casing logic to use a single-buffer allocation strategy, significantly reducing heap allocations.
-   **Improved Panic Safety**: Removed panics when receiving empty strings.
-   **Naming Convention**: Standardized trait methods with the `to_` prefix (e.g., `to_camel_case`).
