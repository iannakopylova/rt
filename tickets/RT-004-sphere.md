# RT-004: Sphere primitive

| Field | Value |
|-------|-------|
| **Status** | To Do |
| **Assignee** | @sofia |
| **Priority** | P1 |
| **Epic** | objects |

## Description

Implement a sphere defined by center and radius. Ray–sphere intersection returns hit distance, surface normal, and material.

## Acceptance criteria

- [ ] Sphere position configurable (e.g. center at `(1, 1, 1)`)
- [ ] Correct ray–sphere intersection (nearest positive `t`)
- [ ] Implements shared `Object` / `Hittable` trait used by tracer
- [ ] Renders as a visible circle/disc in a minimal test scene

## Dependencies

- Blocks: RT-011, RT-013
- Blocked by: RT-002, RT-009 (trait can be stubbed in parallel)

## Notes

Standard quadratic formula; watch for rays inside the sphere.
