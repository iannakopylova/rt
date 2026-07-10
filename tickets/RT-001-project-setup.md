# RT-001: Cargo project & repo structure

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @iana |
| **Priority** | P0 |
| **Epic** | foundation |

## Description

Initialize the Rust project with Cargo and a clean module layout that three developers can work on in parallel without merge conflicts.

Suggested layout:

```
src/
  main.rs
  vec3.rs
  ray.rs
  camera.rs
  light.rs
  material.rs
  scene.rs
  tracer.rs
  ppm.rs
  objects/
    mod.rs
    sphere.rs
    cube.rs
    plane.rs
    cylinder.rs
```

## Acceptance criteria

- [x] `Cargo.toml` exists; `cargo build` succeeds
- [x] Module skeleton in place (empty stubs OK)
- [x] `cargo run` prints a placeholder message (not PPM yet)
- [x] README with one-line project description

## Dependencies

- Blocks: RT-002, RT-003, RT-004–007, RT-009
- Blocked by: —

## Notes

Keep dependencies minimal (stdlib only unless the team agrees otherwise).
