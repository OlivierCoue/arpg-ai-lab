# AGENTS.md

## Project

Small Diablo-like ARPG built with Rust and Bevy.

The project is primarily an experiment in AI-assisted and agentic software development.

## Development

Use stable Rust.

Before considering a task complete:

- `cargo fmt --check`
- `cargo check`
- `cargo clippy`
- `cargo test`

## Git

Keep changes focused on the current task.

Do not modify unrelated files.

Do not remove or weaken tests to make CI pass.

## Architecture

Keep gameplay logic separated from presentation/rendering where practical.

Prefer small, focused systems and modules over large systems.

## Dependencies

Do not add dependencies unless they are necessary for the task.

Always add dependcies to the root Cargo.toml and use thems in member crates with `{ workspace = true }`

Always work with bevy 0.19
