# RT-004: Sphere primitive

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @sofia |
| **Priority** | P1 |
| **Epic** | objects |

## Description

Implement a sphere defined by center and radius. Ray–sphere intersection returns hit distance, surface normal, and material.

## Acceptance criteria

- [x] Sphere position configurable (e.g. center at `(1, 1, 1)`)
- [x] Correct ray–sphere intersection (nearest positive `t`)
- [x] Implements shared `Object` / `Hittable` trait used by tracer
- [x] Renders as a visible circle/disc in a minimal test scene

## Dependencies

- Blocks: RT-011, RT-013
- Blocked by: RT-002, RT-009 (trait can be stubbed in parallel)

## Notes

Standard quadratic formula; watch for rays inside the sphere.

Implemented in `src/objects/sphere.rs` + shared API in `src/objects/mod.rs`:
- `Hittable::hit(ray, t_min, t_max) -> Option<HitRecord>` (RT-009 contract stub)
- half-b quadratic; face-oriented normals; `Material` albedo
- camera-grid disc test in unit tests
