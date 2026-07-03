# Contributing

## Development approach

- **TDD**: tests required for new features and bug fixes. Bug fixes need a regression test that would fail before the fix.
- **Keep it simple**: minimal useful complexity. Small modules with clear responsibilities.

## Getting started

```bash
cargo test
cargo build
```

## Project structure

- `pinterest-dl-core/` — URL extraction, download logic, event system (no GUI/CLI dependencies)
- `pinterest-dl/` — thin CLI wrapper around core
- `pinterest-dl-gui/` — egui-based native GUI

Dependencies flow one way: `gui` → `core`, `cli` → `core`.

## Pull request guidelines

- One change per PR.
- Keep diffs small and focused.
- Update or add tests.
- Run `cargo test` before opening.
- Match the existing code style.

## Code style

- Follow `cargo fmt` conventions.
- No `unsafe` unless absolutely necessary and justified.
- Avoid `unwrap()` in production code; propagate errors instead.
