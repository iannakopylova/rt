# RT-002: Vec3, Ray, Color types

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @iana |
| **Priority** | P0 |
| **Epic** | foundation |

## Description

Implement core math types used everywhere in the ray tracer: 3D vectors, rays, and RGB colors.

## Acceptance criteria

- [x] `Vec3` with add, sub, mul, dot, cross, length, normalize
- [x] `Ray` with `origin` and `direction` (normalized)
- [x] `Color` as RGB `f64` or `Vec3`, with clamp 0–1 and conversion to `u8` (0–255)
- [x] Unit tests for basic vector operations

## Dependencies

- Blocks: RT-003, RT-004–009
- Blocked by: RT-001

## Notes

Consider a shared `HitRecord` struct later in RT-009; not required in this ticket.
