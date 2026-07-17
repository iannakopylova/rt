# RT-012: Scene 2 — plane + cube, dimmer light

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @andriana |
| **Priority** | P1 |
| **Epic** | scenes |

## Description

Second audit scene: flat plane and cube with **lower brightness** than Scene 1.

## Acceptance criteria

- [x] One plane + one cube, positions configurable
- [x] Light brightness clearly lower than Scene 1
- [x] Shadows present
- [x] Final image: **800×600** PPM (e.g. `scenes/scene2_plane_cube.ppm`)

## Dependencies

- Blocks: —
- Blocked by: RT-005, RT-006, RT-008, RT-010

## Notes

Brightness: Scene 1 = `1.0`, Scene 2 = `0.45` (documented in `scenes/README.md`).
CLI: `--scene 2`. Knobs in `scene2_plane_cube` (`src/scenes.rs`).
