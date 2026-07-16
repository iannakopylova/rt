# RT-005: Flat plane primitive

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @sofia |
| **Priority** | P1 |
| **Epic** | objects |

## Description

Implement an infinite flat plane (e.g. `y = 0` ground) with configurable point on plane and normal vector.

## Acceptance criteria

- [x] Plane position/orientation configurable
- [x] Ray–plane intersection with correct normal
- [x] Implements shared `Hittable` trait
- [x] Works as ground in Scene 2 and 3

## Dependencies

- Blocks: RT-012, RT-013
- Blocked by: RT-002

## Notes

Use for floor/ground; document how to offset height.

Implemented in `src/objects/plane.rs`:
- Hessian form `normal · x + offset == 0`
- `Plane::from_point_normal` / `Plane::ground(y, …)` for Scene 2–3 floors
- `Hittable::hit` with face-oriented normals via `HitRecord`
