# Code Review Skill

## Purpose

Review pull requests for this repository as a senior Rust and Bevy engineer.

The goal is to identify correctness issues, architectural problems, regressions, missing tests, and unnecessary complexity before the pull request is merged.

Do not modify files, create commits, or implement fixes during a code review.

---

## Review Priority

Review findings in this order:

1. Correctness and bugs
2. Bevy API correctness and version compatibility
3. Architecture and separation of responsibilities
4. Tests and testability
5. Performance and unnecessary allocations
6. Error handling and robustness
7. Maintainability and readability
8. Scope and unnecessary changes
9. Style and minor improvements

Do not report purely stylistic issues unless they materially affect maintainability.

---

## Bevy Version

This project uses **Bevy 0.19.1**.

Never assume an API exists based on knowledge of another Bevy version.

When reviewing Bevy code:

1. Verify the API against the locally installed Bevy 0.19.1 source when available.
2. Prefer the exact Bevy 0.19.1 documentation.
3. Prefer official Bevy examples from the `release-0.19.1` branch.
4. Treat examples from `main`, `master`, or other Bevy releases as potentially incompatible.

Official documentation:

https://docs.rs/bevy/0.19.1/bevy/

Official examples:

https://github.com/bevyengine/bevy/tree/release-0.19.1/examples

If an API cannot be verified, do not confidently claim that it exists.

---

## Rust Review

Check for:

* unnecessary cloning or allocations
* incorrect ownership or borrowing patterns
* unnecessary `Arc`, `Rc`, `Mutex`, or interior mutability
* incorrect lifetimes
* unnecessary `unwrap()` or `expect()`
* panics on normal runtime paths
* incorrect error propagation
* inefficient data structures
* accidental O(n²) or worse algorithms
* unnecessary synchronization
* code that makes future testing difficult

Do not flag `unwrap()` or `expect()` automatically. Consider whether failure is genuinely impossible or represents a programmer invariant.

---

## Bevy / ECS Review

Check that ECS responsibilities are clearly separated.

Pay particular attention to:

* component responsibilities
* resource responsibilities
* event/message usage
* system responsibilities
* system ordering
* queries
* mutable query conflicts
* entity lifecycle
* startup/setup systems
* plugin boundaries
* state transitions
* unnecessary global resources
* unnecessary coupling between gameplay and rendering

Avoid introducing ECS abstractions merely for abstraction's sake.

Prefer simple Bevy-native solutions when they are sufficient.

---

## Game Architecture

The project should keep gameplay logic as independent as reasonably possible from presentation and rendering.

Prefer a structure similar to:

Input
↓
Intent / Command
↓
Gameplay Logic
↓
Game State / Events
↓
Presentation

This is a guideline, not an absolute requirement.

Do not request architectural changes merely because code does not perfectly match this model.

Only report architectural issues when the current implementation creates meaningful coupling, testing problems, or future maintenance problems.

---

## Time and Simulation

Be particularly careful with time-dependent systems.

Check for:

* accidental frame-rate dependence
* applying delta time more than once
* confusing seconds with world units
* inconsistent time units
* mixing fixed and variable timestep logic
* movement or simulation behavior changing with FPS
* inappropriate use of frame time in gameplay systems

Whenever a helper receives or transforms `delta` time, verify its semantics carefully.

A helper should make it clear whether it expects:

* raw delta time
* elapsed time
* a speed
* a distance
* or a value already multiplied by delta time

Avoid APIs that make double application of delta time easy.

---

## Testing

For every meaningful gameplay change, check whether appropriate tests exist.

Prefer testing gameplay rules independently from rendering whenever practical.

Check:

* normal behavior
* boundary conditions
* invalid states
* edge cases
* deterministic behavior
* regression coverage for fixed bugs

Do not require tests for trivial wiring or purely visual changes unless the behavior is difficult to validate otherwise.

When a PR changes existing behavior, check whether existing tests still describe the intended behavior.

---

## Performance

This is an online ARPG project, so avoid introducing unnecessary per-frame costs.

Pay particular attention to:

* allocations inside frequently running systems
* repeated entity/world searches
* unnecessary query iteration
* expensive operations performed every frame
* excessive logging
* unnecessary cloning
* avoidable synchronization
* systems that scale poorly with entity count

However, do not prematurely optimize simple code.

A measurable or clearly foreseeable problem should be preferred over speculative micro-optimizations.

---

## Pull Request Scope

Verify that the implementation matches the issue being solved.

Flag:

* unrelated refactors
* unrelated dependency changes
* unnecessary architecture changes
* generated files that should not be committed
* temporary debugging code
* changes unrelated to the acceptance criteria

Do not request unrelated improvements simply because they could theoretically be useful later.

---

## Documentation Consistency

Check that documentation and project instructions remain consistent with the actual implementation.

In particular, verify consistency between:

* `AGENTS.md`
* `Cargo.toml`
* workspace dependencies
* Bevy version
* documented architecture
* CI commands
* development environment

If documentation becomes incorrect because of the PR, report it.

---

## CI and Validation

A PR should pass the project's standard validation commands.

Check whether the changes are compatible with:

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

If CI already performs these checks, do not duplicate findings merely because the commands are not run locally.

When possible, use the repository's existing validation scripts rather than inventing alternative commands.

---

## Finding Severity

Only report actionable findings.

### High

Use for:

* correctness bugs
* crashes
* data corruption
* security problems
* serious gameplay or simulation errors
* issues that make the PR fundamentally unsafe to merge

### Medium

Use for:

* real bugs under specific conditions
* incorrect behavior
* significant architectural problems
* missing important tests
* compatibility problems
* performance problems with meaningful impact

### Low

Use for:

* minor correctness concerns
* maintainability problems
* small but worthwhile improvements

Do not report speculative concerns as Medium or High.

---

## Review Comments

Each finding should:

1. Identify the concrete problem.
2. Explain why it matters.
3. Explain when it can occur.
4. Suggest a direction for fixing it when useful.
5. Reference the relevant code or behavior.

Prefer concrete explanations over generic advice.

Bad:

> This could be improved.

Good:

> `movement_speed` is already multiplied by `delta_secs` before being passed to `apply_movement`, but `apply_movement` multiplies the value by `delta_secs` again. This makes movement scale with the square of the frame duration. Keep the helper responsible for either applying delta time or receiving a distance-per-frame value, but not both.

---

## Avoid False Positives

Before reporting a finding:

* Check the surrounding code.
* Check how the value is produced and consumed.
* Check system ordering when relevant.
* Check existing tests.
* Check the actual Bevy 0.19.1 API.
* Consider whether the behavior is intentional.
* Prefer evidence from the repository over assumptions.

Do not report hypothetical problems without a concrete failure mode.

Do not request changes solely because another implementation would be more idiomatic.

---

## Final Review

At the end of the review:

* Summarize the important findings.
* Distinguish blocking issues from optional improvements.
* Do not approve or reject based solely on style.
* Do not propose unrelated future work.

The review should help the implementer make the smallest set of changes necessary to produce a correct, maintainable pull request.
