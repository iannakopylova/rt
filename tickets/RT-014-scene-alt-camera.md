# RT-014: Scene 4 — same scene, new camera

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @andriana |
| **Priority** | P1 |
| **Epic** | scenes |

## Description

Fourth audit scene: **identical objects** as Scene 3, but camera moved to a different position/angle.

## Acceptance criteria

- [x] Same object layout as Scene 3
- [x] Camera position/angle clearly different (different perspective)
- [x] Final image: **800×600** PPM (e.g. `scenes/scene4_alt_camera.ppm`)

## Dependencies

- Blocks: —
- Blocked by: RT-003, RT-013

## Notes

`scene4_alt_camera` = `scene3_world()` + `scene3_camera_alt`.
CLI: `--scene 4`.
