# RT-011: Scene 1 — sphere only (800×600)

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @andriana |
| **Priority** | P1 |
| **Epic** | scenes |

## Description

Build and export the first mandatory audit image: a scene containing only a sphere.

## Acceptance criteria

- [x] One sphere with configurable position
- [x] Lighting with shadows visible on ground or background
- [x] Final image: **800×600** PPM saved (e.g. `scenes/scene1_sphere.ppm`)
- [x] Reproducible via CLI or documented `cargo run` command

## Dependencies

- Blocks: —
- Blocked by: RT-004, RT-008, RT-010

## Notes

Ground plane is included so the sphere casts a visible shadow (not counted as a second “subject”).
Knobs live in `src/scenes.rs` (`scene1_sphere`). CLI: `--scene 1`.
