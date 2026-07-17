# RT-013: Scene 3 — all four objects

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @andriana |
| **Priority** | P1 |
| **Epic** | scenes |

## Description

Third audit scene: one cube, one sphere, one cylinder, and one flat plane in the same scene.

## Acceptance criteria

- [x] All four primitives visible and correctly lit
- [x] Object positions configurable before render
- [x] Final image: **800×600** PPM (e.g. `scenes/scene3_all.ppm`)

## Dependencies

- Blocks: RT-014
- Blocked by: RT-004–007, RT-008, RT-010

## Notes

World lives in `scene3_world()`; front camera in `scene3_camera_front()`.
RT-014 should reuse the world and only change the camera.
CLI: `--scene 3`.
