# RT-003: Camera (position, angle, FOV)

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @sofia |
| **Priority** | P0 |
| **Epic** | foundation |

## Description

Implement a pinhole camera that generates a ray for each pixel `(x, y)` given image width, height, position, look-at point, and field of view.

## Acceptance criteria

- [x] Camera configurable: position, look-at (or direction), up vector, FOV
- [x] `get_ray(u, v) -> Ray` for normalized coords or pixel indices
- [x] Changing camera position produces visibly different rays (test or debug print)
- [x] Documented example: move camera for Scene 4

## Dependencies

- Blocks: RT-009, RT-014
- Blocked by: RT-002

## Notes

Use a right-handed coordinate system and document which axis is “up”.

Implemented in `src/camera.rs`:
- `Camera::look_at(eye, look_at, world_up, vfov_degrees, aspect_ratio)`
- `get_ray(u, v)` with `u,v ∈ [0,1]` (v=0 bottom, v=1 top)
- `ray_through_pixel(x, y, width, height)` for PPM scan order
- Module docs include Scene 4 eye-move example; +Y is world up
