# AGENTS.md

## Project

This is a small Diablo-like ARPG built with Rust and Bevy.

The project is also an experiment in AI-assisted and agentic software development.

Agents are expected to work as software engineers: inspect the repository, verify assumptions, make focused changes, run validation, and report what was actually verified.

---

## Superpowers

This project uses the Superpowers skills workflow.

Superpowers must be used for every non-trivial coding task.

Before making code changes:

1. Check the available Superpowers skills.
2. Determine which skill or skills apply to the task.
3. Read and follow the relevant skill instructions.
4. Complete the required planning, brainstorming, debugging, or testing workflow before implementation.

### Required skill selection

Use the appropriate workflow depending on the task:

* New feature or significant change → `brainstorming` → `writing-plans` → implementation
* Multi-step implementation → `writing-plans`
* Bug or unexpected behavior → `systematic-debugging`
* New functionality requiring tests → `test-driven-development`
* Code review → appropriate review workflow
* Refactoring → appropriate planning/testing workflow

If unsure which skill applies, inspect the available Superpowers skills before proceeding.

Do not merely mention a Superpowers skill. Actually read and follow its instructions.

---

## Bevy Version

This project uses **Bevy 0.19.1**.

This version is a strict requirement.

All Bevy-related code, APIs, examples, dependencies, and documentation must be compatible with Bevy 0.19.1.

Do not use APIs from:

* Bevy 0.18
* Bevy 0.17
* Bevy 0.16
* older versions
* Bevy `main`
* Bevy development versions

Do not assume that an API exists because it existed in a previous Bevy version.

---

## Bevy Source of Truth

When working with Bevy, never rely solely on internal model knowledge.

The authoritative sources are, in this order:

1. The exact Bevy 0.19.1 source installed in the local Cargo registry.
2. Bevy 0.19.1 Rust documentation:
   https://docs.rs/bevy/0.19.1/bevy/
3. Official Bevy 0.19.1 examples:
   https://github.com/bevyengine/bevy/tree/release-0.19.1/examples

The local installed source and the resolved dependency version are especially important because they represent the exact version used by this repository.

Never use Bevy examples from `main` or another release.

---

## Mandatory Bevy Verification

For every task involving Bevy APIs:

1. Inspect `Cargo.toml` and `Cargo.lock`.
2. Confirm the actual resolved Bevy version.
3. Identify the Bevy API required by the task.
4. Verify that API against the locally installed Bevy 0.19.1 source.
5. If useful, verify the usage against the official Bevy 0.19.1 examples.
6. Only then write the implementation.

If an API cannot be verified, do not invent or guess it.

If remembered knowledge conflicts with the local Bevy 0.19.1 source, trust the local source.

### Anti-hallucination rule

Do not write Bevy code based on familiarity with another Bevy version.

For example, do not assume that a previously known:

* component
* bundle
* system API
* query API
* schedule API
* event API
* resource API
* rendering API
* input API
* asset API

still exists in Bevy 0.19.1.

Verify it first.

---

## Bevy Implementation Workflow

For any non-trivial Bevy task:

1. Read the relevant Superpowers skill.
2. Inspect the existing project structure and code.
3. Inspect `Cargo.toml` and `Cargo.lock`.
4. Verify the exact Bevy version.
5. Inspect the local Bevy 0.19.1 source for the APIs required.
6. Check the corresponding Bevy 0.19.1 examples when useful.
7. Create the implementation plan.
8. Implement the smallest focused change that satisfies the requirements.
9. Run formatting.
10. Run compilation checks.
11. Run Clippy.
12. Run tests.
13. Fix any issues discovered by validation.
14. Report the validation actually performed.

Do not skip API verification merely because the task appears simple.

---

## Validation

Before considering a task complete, run:

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

If the repository provides a project validation script, prefer using that script.

Never remove, weaken, or bypass tests or compiler checks just to make the task pass.

If validation fails, investigate and fix the underlying problem.

Do not hide or ignore compiler warnings.

---

## Development

Use stable Rust.

Prefer idiomatic, simple Rust over clever abstractions.

Keep functions, systems, and modules focused.

Avoid unnecessary dependencies.

Do not introduce an abstraction unless it provides a clear benefit.

Do not prematurely optimize.

Prefer code that is easy for another developer or agent to understand and modify.

---

## Architecture

The project is a Diablo-like ARPG.

Keep gameplay logic separated from presentation/rendering where practical.

Prefer the following conceptual separation:

```text
Input
  ↓
Game Commands / Intent
  ↓
Game Logic
  ↓
Game State / Events
  ↓
Presentation
```

Gameplay rules should be testable without requiring the renderer whenever practical.

Avoid putting substantial gameplay logic directly inside rendering or input systems.

Prefer small, focused Bevy systems over large systems containing unrelated responsibilities.

Do not introduce large global state structures when smaller components, resources, or focused modules are more appropriate.

---

## Dependencies

Avoid adding dependencies unless they are necessary for the task.

All shared dependencies must be declared in the workspace root `Cargo.toml`.

Member crates must reference workspace dependencies.

Example:

Root `Cargo.toml`:

```toml
[workspace.dependencies]
bevy = "0.19.1"
```

Member crate:

```toml
[dependencies]
bevy = { workspace = true }
```

Do not independently specify versions for workspace-managed dependencies in member crates.

When adding a Bevy-related dependency:

1. Verify that its version is compatible with Bevy 0.19.1.
2. Add it to the workspace root.
3. Reference it from the member crate using `{ workspace = true }`.

Do not upgrade or downgrade Bevy-related dependencies unless explicitly requested.

---

## Git

Keep changes focused on the current task.

Do not modify unrelated files.

Do not rewrite unrelated code.

Do not make broad refactors while implementing a focused feature.

Prefer one focused branch and pull request per task.

Use clear commit messages.

Before creating a pull request:

```bash
git diff
git status
```

Review the changes and make sure they correspond to the requested task.

---

## Issue and Pull Request Scope

Treat the GitHub issue as the contract for the implementation.

Before coding:

* understand the issue requirements;
* identify acceptance criteria;
* identify constraints;
* identify what is explicitly out of scope.

Do not expand the scope without justification.

If the issue is ambiguous or technically impossible as written, explain the ambiguity before making substantial changes.

---

## Agent Behavior

Agents should:

* inspect before modifying;
* verify assumptions;
* make focused changes;
* prefer existing project patterns;
* use the compiler and tests as feedback;
* investigate failures rather than guessing;
* report what was actually verified.

Agents should not:

* invent APIs;
* rely on outdated Bevy knowledge;
* perform unrelated refactors;
* add unnecessary dependencies;
* remove tests to make CI pass;
* claim that documentation or source was checked when it was not;
* claim that tests passed when they were not run.

When uncertain, investigate the repository and its dependencies before making a decision.
