# Style

[![CI](https://github.com/hvpaiva/style/actions/workflows/ci.yml/badge.svg)](https://github.com/hvpaiva/style/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/hvpaiva/style/branch/main/graph/badge.svg)](https://codecov.io/gh/hvpaiva/style)
[![Crates.io](https://img.shields.io/crates/v/style.svg)](https://crates.io/crates/style)
[![docs.rs](https://docs.rs/style/badge.svg)](https://docs.rs/style)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A declarative terminal styling library for Rust.

> **Status**: Early development (MVP)

## Features (planned)

- Composable style builder with fluent API
- Text formatting (bold, italic, underline, strikethrough, dim, reverse)
- Color support (ANSI, ANSI256, TrueColor) with automatic degradation
- Horizontal padding and margin
- Style inheritance and composition

## Development

```bash
# Setup development environment
just setup

# Run all checks
just check

# Format, lint, test individually
just fmt
just lint
just test

# Generate coverage report
just coverage
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

[MIT](LICENSE)

## Acknowledgments

Inspired by [Lip Gloss](https://github.com/charmbracelet/lipgloss).
