# RT-005: Flat plane primitive

| Field | Value |
|-------|-------|
| **Status** | To Do |
| **Assignee** | @sofia |
| **Priority** | P1 |
| **Epic** | objects |

## Description

Implement an infinite flat plane (e.g. `y = 0` ground) with configurable point on plane and normal vector.

## Acceptance criteria

- [ ] Plane position/orientation configurable
- [ ] Ray–plane intersection with correct normal
- [ ] Implements shared `Hittable` trait
- [ ] Works as ground in Scene 2 and 3

## Dependencies

- Blocks: RT-012, RT-013
- Blocked by: RT-002

## Notes

Use for floor/ground; document how to offset height.

## Branch

`ticket/RT-005-plane`
