Implement GitHub issue #2 according to AGENTS.md.

Create the feature branch, implement and validate the changes, commit them, push the branch, and create a PR targeting main with "Closes #2".

Do not merge the PR.

------------------

Review the latest Copilot Code Review comments on the current pull request.

For each unresolved finding:

1. Inspect the relevant code and surrounding context.
2. Verify whether the finding is actually valid.
3. If valid, implement the smallest appropriate fix.
4. If invalid, explain why it is a false positive instead of changing the code.
5. Add or update tests when appropriate.
6. Verify all changes against the project's `AGENTS.md` rules, especially Bevy 0.19.1 compatibility.
7. Run the project's validation commands:

   * cargo fmt --all --check
   * cargo check --workspace
   * cargo clippy --workspace --all-targets -- -D warnings
   * cargo test --workspace

Do not make unrelated changes.

Do not create a new branch. Work on the current PR branch.

Do not commit or push unless explicitly asked.
