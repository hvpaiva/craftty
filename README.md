# Craftty

[![CI](https://github.com/hvpaiva/craftty/actions/workflows/ci.yml/badge.svg)](https://github.com/hvpaiva/craftty/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/hvpaiva/craftty/branch/main/graph/badge.svg)](https://codecov.io/gh/hvpaiva/craftty)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A modular terminal toolkit for Rust.

> **Status**: Early development

## Crates

| Crate | Description | Version |
|-------|-------------|---------|
| [craftty-ink](crates/craftty-ink) | Declarative terminal styling library | [![Crates.io](https://img.shields.io/crates/v/craftty-ink.svg)](https://crates.io/crates/craftty-ink) |
| [craftty](crates/craftty) | CLI toolkit | [![Crates.io](https://img.shields.io/crates/v/craftty.svg)](https://crates.io/crates/craftty) |

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

Inspired by [Lip Gloss](https://github.com/charmbracelet/lipgloss) and the [Charm](https://charm.sh) ecosystem.
