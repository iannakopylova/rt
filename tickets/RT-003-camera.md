# RT-003: Camera (position, angle, FOV)

| Field | Value |
|-------|-------|
| **Status** | To Do |
| **Assignee** | @sofia |
| **Priority** | P0 |
| **Epic** | foundation |

## Description

Implement a pinhole camera that generates a ray for each pixel `(x, y)` given image width, height, position, look-at point, and field of view.

## Acceptance criteria

- [ ] Camera configurable: position, look-at (or direction), up vector, FOV
- [ ] `get_ray(u, v) -> Ray` for normalized coords or pixel indices
- [ ] Changing camera position produces visibly different rays (test or debug print)
- [ ] Documented example: move camera for Scene 4

## Dependencies

- Blocks: RT-009, RT-014
- Blocked by: RT-002

## Notes

Use a right-handed coordinate system and document which axis is “up”.

## Branch

`ticket/RT-003-camera`
