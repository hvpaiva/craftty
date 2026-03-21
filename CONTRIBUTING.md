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

## Project Structure

This is a Cargo workspace monorepo. All crates live under `crates/`:

```
crates/
├── craftty-ink/   # Declarative terminal styling library (core)
├── craftty/       # CLI toolkit (the only binary crate)
└── ...            # Future crates (all libraries)
```

Each crate has its own `README.md` and `CHANGELOG.md`. Workspace-level metadata (edition, authors, license) is shared via `Cargo.toml` at the root.

## Conventional Commits

This project enforces [Conventional Commits](https://www.conventionalcommits.org/) via git hooks.

You can use `cog commit` (installed by `just setup`) for an interactive guide through the format:

```bash
cog commit feat "add color degradation support" craftty-ink
```

Or use `git commit` directly — the commit-msg hook validates the format either way.

Format:

```
type(scope): description

[optional body]

[optional footer(s)]
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`

**Scopes:** `craftty-ink`, `craftty` (optional, but only these are accepted)

**Examples:**

```
feat(craftty-ink): add color degradation support
fix(craftty): handle missing terminal width
docs: update README with usage examples
chore: update CI workflow
```

## Automated Versioning

Versions and changelogs are managed automatically by [cocogitto](https://docs.cocogitto.io/) via the CD pipeline. **Do not** edit `CHANGELOG.md` files or `version` fields in `Cargo.toml` manually — they will be overwritten on the next release.

A pre-commit hook and a CI check enforce this. If you have a legitimate reason to edit these files (e.g. structural changes, migrations):

1. **Locally:** `SKIP_RELEASE_GUARD=1 git commit ...`
2. **CI:** add the `release-override` label to the PR

## Development

```bash
just check      # all checks (fmt, lint, test, audit)
just fmt        # format code
just lint       # clippy
just test       # tests
just audit      # dependency audit
just run        # run the craftty CLI
just hooks      # install/update git hooks
```

## Code Style

- Follow idiomatic Rust
- Run `cargo fmt` before committing
- All public APIs must be documented
- Write tests for new functionality

## Reporting Issues

Before opening an issue, check if a similar one already exists. Include reproduction steps and the affected crate when reporting bugs.
