# Bevy ECS

## Purpose

This skill defines safe and idiomatic patterns for working with Bevy's Entity Component System.

The project uses **Bevy 0.19.1**.

This skill is especially important when working with:

* `Query`
* `With` / `Without`
* mutable component access
* multiple queries in the same system
* `Res` / `ResMut`
* `Event` / `Message`
* system scheduling
* system ordering
* entity relationships
* component access conflicts

Always verify Bevy 0.19.1 APIs against the project's configured sources of truth.

---

## 1. ECS Query Access Safety

Bevy performs ECS query access validation at runtime.

Rust's borrow checker cannot detect every conflict between separate Bevy queries because the queries describe ECS access rather than ordinary Rust references.

Successful compilation does **not** guarantee that multiple queries in a system are ECS-access compatible.

### Multiple queries accessing the same component

Pay particular attention when multiple queries access the same component.

For example:

```rust
fn camera_follow_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
) {
    // ...
}
```

These queries access `Transform` with different mutability.

Even if the game's entity design guarantees that a player and camera are different entities, Bevy cannot necessarily infer that from the marker components.

The queries must explicitly establish that they cannot match the same entity.

For example:

```rust
fn camera_follow_system(
    player_query: Query<
        &Transform,
        (With<Player>, Without<GameCamera>),
    >,
    mut camera_query: Query<
        &mut Transform,
        (With<GameCamera>, Without<Player>),
    >,
) {
    // ...
}
```

### Rule

When multiple queries in the same system access the same component:

1. Identify the access type of every query.
2. Determine whether the queries could theoretically match the same entity.
3. If the accesses conflict, explicitly establish disjointness.
4. Use `With<T>`, `Without<T>`, or another appropriate ECS mechanism.
5. Do not rely solely on game-level assumptions about which components an entity "should" have.

Think in terms of **possible ECS matches**, not intended game semantics.

---

## 2. Mutable Query Access

Mutable queries require particular care.

Examples:

```rust
Query<&mut Transform>
Query<(&mut Transform, &mut Velocity)>
Query<&mut Health>
```

Before adding another query to a system, check whether it accesses any of the same components.

For example, this should trigger an explicit safety analysis:

```rust
fn system(
    query_a: Query<&Transform, With<Player>>,
    query_b: Query<&mut Transform, With<Enemy>>,
) {
}
```

Ask:

* Can one entity satisfy both filters?
* If yes, the queries conflict.
* If no, is that disjointness explicitly represented in the query filters?
* If Bevy cannot prove the queries are disjoint, add the appropriate filters or restructure the system.

Do not assume that marker components are mutually exclusive merely because the game's spawning code currently treats them that way.

---

## 3. Prefer `Without` to Encode Disjointness

When two queries are logically disjoint, express that directly in the query.

Example:

```rust
Query<&Transform, (With<Player>, Without<GameCamera>)>
```

and:

```rust
Query<&mut Transform, (With<GameCamera>, Without<Player>)>
```

This documents the invariant as part of the ECS query itself.

It also protects the system if entity construction changes later.

Avoid comments such as:

```rust
// Player and camera can never be the same entity.
```

when the query itself could express that constraint.

Prefer making the invariant executable.

---

## 4. Query Design

Keep queries focused.

Prefer:

```rust
Query<(&Transform, &Velocity), With<Player>>
```

over querying many unrelated components that the system does not need.

For mutable queries, request only the components that must actually be mutated.

Avoid unnecessarily broad queries such as:

```rust
Query<(
    &mut Transform,
    &mut Velocity,
    &Health,
    &Inventory,
    &PlayerStats,
)>
```

if the system only needs `Transform` and `Velocity`.

Smaller queries:

* make access conflicts easier to reason about;
* reduce coupling;
* make systems easier to test;
* make intent clearer.

---

## 5. Querying the Same Entity in Different Ways

When the logic intentionally needs multiple views of the same entity, consider whether the system should use a single query instead.

Instead of:

```rust
Query<&Transform, With<Player>>
Query<&Velocity, With<Player>>
```

prefer:

```rust
Query<(&Transform, &Velocity), With<Player>>
```

when both pieces of data belong to the same entity and are used together.

This reduces unnecessary query complexity.

---

## 6. `single` and Entity Cardinality

When a system expects exactly one entity, make that assumption explicit.

For example, a camera-follow system may expect exactly one player:

```rust
let player = player_query.single();
```

However, verify the exact Bevy 0.19.1 API before using it.

Do not silently assume a query has exactly one result if the game architecture does not guarantee it.

If zero or multiple entities are valid states, handle that explicitly.

Do not use query cardinality methods merely to silence an error.

---

## 7. Query Iteration and Mutation

Prefer normal query iteration when multiple entities are expected:

```rust
for mut transform in &mut query {
    // ...
}
```

When modifying entities, ensure the query has the minimum mutable access required.

Avoid unnecessary mutable queries:

```rust
Query<&mut Transform>
```

when the system only needs to read:

```rust
Query<&Transform>
```

Reducing mutable access makes ECS scheduling and query compatibility easier to reason about.

---

## 8. System Access Conflicts vs Scheduling

Distinguish between:

1. **Query access conflicts inside one system**
2. **System ordering or scheduling between separate systems**

For example:

```text
System A
    mutates Transform

System B
    reads Transform
```

This is different from:

```text
System B
    Query<&Transform>

System B
    Query<&mut Transform>
```

The first is a scheduling concern.

The second is a query access concern within the same system.

Do not attempt to solve a query conflict by arbitrarily changing system ordering.

---

## 9. ECS Runtime Validation

Bevy may detect invalid query combinations when systems are initialized or executed.

Therefore:

```text
cargo check
cargo clippy
```

are necessary but not sufficient to validate ECS query safety.

For ECS changes, also consider whether the affected systems actually need to be executed during testing or manual verification.

When practical, add a regression test for important ECS invariants.

For example, if an invariant depends on two kinds of entities always being disjoint, consider whether the relevant system can be exercised in a Bevy app test.

---

## 10. Entity Design

Do not make ECS systems depend on undocumented assumptions about entity composition.

Bad:

```text
"A player entity will never have GameCamera because we don't spawn it that way."
```

Better:

```text
"The player query explicitly excludes GameCamera."
```

The second form makes the assumption visible and machine-checkable.

However, do not blindly add `Without` filters everywhere.

First determine whether the queries actually have overlapping access.

The goal is **correct and minimal ECS constraints**, not maximum filtering.

---

## 11. Common Review Checklist

When reviewing or implementing a Bevy ECS system, check:

### Queries

* What components does each query read?
* What components does each query mutate?
* Can two queries match the same entity?
* Are conflicting accesses explicitly disjoint?
* Could `With` / `Without` make the invariant explicit?
* Are queries broader than necessary?

### Entities

* Does the system rely on assumptions about entity composition?
* Are those assumptions encoded in the ECS query?
* Does the query expect exactly one entity?
* What happens if zero or multiple entities exist?

### Systems

* Is the problem actually query access or system scheduling?
* Are mutable accesses necessary?
* Is system ordering explicitly required?

### Runtime

* Can this ECS invariant fail only at runtime?
* Has the affected system actually been executed?
* Is a regression test appropriate?

---

## 12. Bevy Version Requirement

This project uses **Bevy 0.19.1**.

Before using any ECS API:

1. Check `Cargo.toml`.
2. Check `Cargo.lock` if necessary.
3. Verify the API against the local Bevy 0.19.1 source.
4. Check the Bevy 0.19.1 documentation when useful.
5. Check official Bevy 0.19.1 examples when useful.

Never rely on an API remembered from another Bevy version.

If documentation or remembered knowledge conflicts with the installed source, trust the installed Bevy 0.19.1 source.

---

## 13. Completion Criteria

Before considering an ECS task complete:

* Queries have minimal component access.
* Mutable accesses have been explicitly reviewed.
* Conflicting queries are explicitly disjoint when necessary.
* Entity assumptions are represented in ECS filters where appropriate.
* System ordering issues are distinguished from query access issues.
* Relevant runtime behavior has been tested when practical.
* Bevy APIs were verified against version 0.19.1.
* Formatting, compilation, clippy, and tests have been run according to `AGENTS.md`.
