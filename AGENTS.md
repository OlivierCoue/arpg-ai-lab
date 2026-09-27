# AGENTS.md

## Project

This is a small Diablo-like ARPG built with Rust and Bevy.

The project is also an experiment in AI-assisted and agentic software development.

Agents must work like software engineers: inspect the repository, verify assumptions, make focused changes, validate them, and report what was actually verified.

---

## Agent Workflow

### Superpowers

This project uses the Superpowers skills workflow.

Superpowers must be used for every non-trivial coding task.

Before modifying code:

1. Inspect the available Superpowers skills.
2. Select the skill or skills relevant to the task.
3. Read and follow their instructions.
4. Complete the required planning, brainstorming, debugging, or testing workflow before implementation.

Use the appropriate workflow:

* New feature or significant change → `brainstorming` → `writing-plans` → implementation
* Multi-step implementation → `writing-plans`
* Bug or unexpected behavior → `systematic-debugging`
* New functionality requiring tests → `test-driven-development`
* Code review → appropriate review workflow
* Refactoring → appropriate planning/testing workflow

Do not merely mention a skill. Actually read and follow its instructions.

### Skills

Skills are stored under `.github/skills/`.

When a task involves Bevy ECS, queries, components, mutable ECS access, system scheduling, or query access conflicts, read and follow:

`.github/skills/bevy-ecs/SKILL.md`

Do not assume a skill is understood from its name alone. Read the actual `SKILL.md` before implementation.

### Issue Scope

Treat the GitHub issue as the implementation contract.

Before coding:

* understand the requirements;
* identify acceptance criteria;
* identify constraints;
* identify what is explicitly out of scope.

Do not expand the scope without justification.

If the issue is ambiguous or technically impossible as written, explain the ambiguity before making substantial changes.

---

## Bevy

This project uses **Bevy 0.19.1**. This is a strict requirement.

All Bevy code, APIs, examples, dependencies, and documentation must be compatible with Bevy 0.19.1.

Do not use APIs from older Bevy releases, Bevy `main`, or development versions.

### Sources of Truth

When working with Bevy, use these sources in order:

1. The exact Bevy 0.19.1 source installed in the local Cargo registry.
2. Bevy 0.19.1 documentation:
   https://docs.rs/bevy/0.19.1/bevy/
3. Official Bevy 0.19.1 examples:
   https://github.com/bevyengine/bevy/tree/release-0.19.1/examples

Never use examples from `main` or another Bevy release as authoritative.

### API Verification

For every task involving Bevy APIs:

1. Inspect `Cargo.toml` and `Cargo.lock`.
2. Confirm the resolved Bevy version.
3. Identify the APIs required by the task.
4. Verify those APIs against the local Bevy 0.19.1 source.
5. Check the official 0.19.1 examples when useful.
6. Only then implement the code.

If remembered knowledge conflicts with the local source, trust the local source.

Never invent or guess a Bevy API.

---

## Architecture

Keep gameplay logic separated from presentation/rendering where practical.

Prefer this conceptual flow:

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

Prefer small, focused Bevy systems over large systems containing unrelated responsibilities.

Avoid substantial gameplay logic directly inside rendering or input systems.

Do not introduce large global state structures when smaller components, resources, or focused modules are more appropriate.

---

## Rust and Code Quality

Use stable Rust.

Prefer simple, idiomatic Rust over clever abstractions.

Keep functions, systems, and modules focused.

Avoid unnecessary:

* allocations;
* cloning;
* synchronization;
* dependencies;
* abstractions;
* global state;
* per-frame work.

Do not prematurely optimize. Prefer measurable or clearly foreseeable performance improvements.

Code should be easy for another developer or agent to understand and modify.

---

## Dependencies

Avoid adding dependencies unless necessary.

Shared dependencies must be declared in the workspace root `Cargo.toml`.

Member crates must reference workspace dependencies.

Example:

Root:

```toml
[workspace.dependencies]
bevy = "0.19.1"
```

Member crate:

```toml
[dependencies]
bevy = { workspace = true }
```

Do not independently specify versions for workspace-managed dependencies.

When adding a Bevy-related dependency:

1. Verify compatibility with Bevy 0.19.1.
2. Add it to the workspace root.
3. Reference it from the member crate using `{ workspace = true }`.

Do not upgrade or downgrade Bevy-related dependencies unless explicitly requested.

---

## Validation

Before considering a task complete, run:

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

If the repository provides a validation script, prefer using it.

If validation fails, investigate and fix the underlying problem.

Never:

* remove or weaken tests to make validation pass;
* bypass compiler checks;
* ignore warnings;
* claim validation passed when it was not run.

Report the validation that was actually performed.

---

## Git Workflow

Each GitHub issue must be implemented on its own dedicated branch.

### Before implementation

Start from the latest `main`:

```bash
git checkout main
git pull --ff-only
```

Then create a dedicated branch:

```bash
git checkout -b <branch-name>
```

Prefer descriptive names such as:

```text
feature/<short-description>
fix/<short-description>
refactor/<short-description>
```

Include the issue number when available:

```text
feature/2-player-combat
fix/15-player-collision
```

Never implement an issue directly on `main`.

### During implementation

Keep all changes for the issue on its dedicated branch.

Do not modify unrelated files or perform broad refactors.

Before committing:

```bash
git status
git diff
```

Review the complete diff and ensure every change belongs to the current issue.

Run the full project validation before committing.

### Commit

Use focused commits with clear messages.

Prefer conventional commit-style messages when appropriate:

```text
feat: add player combat
fix: prevent duplicate damage
test: add player combat tests
refactor: extract damage calculation
```

Do not create commits containing unrelated changes.

### Push

After validation succeeds:

```bash
git push -u origin <branch-name>
```

### Pull Request

After pushing the branch, create a Pull Request targeting `main`.

The PR must:

* target `main`;
* reference the GitHub issue;
* summarize the implementation;
* mention important design decisions when relevant;
* mention tests and validation performed.

Use GitHub closing syntax when appropriate:

```text
Closes #<issue-number>
```

This allows GitHub to automatically close the issue when the PR is merged.

Do not merge the Pull Request unless explicitly authorized by the user.

### Review Cycle

After creating the PR:

1. Wait for CI and automated code review.
2. Inspect all review findings.
3. Determine whether each finding is valid.
4. Fix valid findings on the same feature branch.
5. Push the fixes.
6. Wait for CI and automated review again.
7. Report the final status to the user.

Do not create a new PR to address review comments.

The human developer retains responsibility for the final merge.

---

## Agent Behavior

Agents should:

* inspect before modifying;
* verify assumptions;
* follow the issue scope;
* follow the Superpowers workflow;
* verify Bevy APIs against the exact installed version;
* prefer existing project patterns;
* make the smallest focused change;
* use the compiler and tests as feedback;
* investigate failures rather than guessing;
* report what was actually verified.

Agents must not:

* invent APIs;
* rely on outdated Bevy knowledge;
* implement directly on `main`;
* perform unrelated refactors;
* add unnecessary dependencies;
* remove tests to make CI pass;
* bypass validation;
* claim that documentation or source was checked when it was not;
* claim that tests passed when they were not run.

When uncertain, investigate the repository and its dependencies before making a decision.
