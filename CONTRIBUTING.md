# Contributing

Thank you for considering contributing to this project!

## Getting Started

1. Fork the repository
2. Clone your fork
3. Run the setup:

```bash
just setup
```

This installs all required tools (cocogitto, cargo-deny, cargo-nextest, just) and configures git hooks.

## Pull Requests

1. Create a branch from `main` in your fork
2. Make your changes
3. Ensure `just check` passes
4. Commit using [Conventional Commits](#conventional-commits)
5. Push to your fork and open a Pull Request against `main`

Keep PRs focused and small when possible. Separate refactoring from functional changes.

## Conventional Commits

This project enforces [Conventional Commits](https://www.conventionalcommits.org/) via git hooks.

```
type(scope): description

[optional body]

[optional footer(s)]
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`

**Examples:**

```
feat: add color degradation support
fix(render): handle empty string input
docs: update README with usage examples
```

## Development

```bash
just check      # all checks (fmt, lint, test, audit)
just fmt        # format code
just lint       # clippy
just test       # tests
just audit      # dependency audit
```

## Code Style

- Follow idiomatic Rust
- Run `cargo fmt` before committing
- All public APIs must be documented
- Write tests for new functionality

## Reporting Issues

Before opening an issue, check if a similar one already exists. Include reproduction steps when reporting bugs.
