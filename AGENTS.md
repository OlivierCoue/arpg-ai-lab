# AGENTS.md

Rust + Bevy 0.19.1 Diablo-like ARPG.

## Bevy

**Version: 0.19.1.**

For any Bevy-related task:
- First inspect `/workspaces/arpg-ai-lab/bevy/examples/`.
- Treat these local examples as the primary reference for Bevy APIs and patterns.
- If a filesystem search does not find the examples, verify the directory with `ls`/filesystem tools before assuming it is absent.
- Do not fall back to remembered Bevy APIs because an example search failed.
- Verify APIs against the local Bevy source when needed.
- Never use examples or APIs from another Bevy version, `main`, or development releases.

## Rules

* Treat GitHub issues as the implementation contract. Stay within scope.
* Use Superpowers for every non-trivial task. Read and follow the relevant skills before coding.
* Read `.github/skills/bevy-ecs/SKILL.md` for Bevy ECS work.
* Verify Bevy APIs against the **local Bevy 0.19.1 source**. Never guess or use APIs from other versions.
* Prefer existing patterns, small focused changes, idiomatic stable Rust, and testable game logic.
* Avoid unnecessary dependencies, abstractions, allocations, cloning, global state, and per-frame work.
* Keep gameplay separate from input/rendering where practical.
* Never work directly on `main`; use one branch per issue.
* Do not modify unrelated code or perform broad refactors.
* Before completion, run relevant validation; normally:
  `cargo fmt --all --check`
  `cargo check --workspace`
  `cargo clippy --workspace --all-targets -- -D warnings`
  `cargo test --workspace`
* Never weaken tests, bypass checks, or claim validation that was not performed.
* Review `git diff` before committing. Push only after validation. PRs target `main`; never merge without explicit authorization.
* Investigate failures instead of guessing and report only what was actually verified.
