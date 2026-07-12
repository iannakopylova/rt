# RT-006: Cube primitive

| Field | Value |
|-------|-------|
| **Status** | To Do |
| **Assignee** | @sofia |
| **Priority** | P1 |
| **Epic** | objects |

## Description

Implement an axis-aligned cube (AABB) with configurable min/max corners or center + size.

## Acceptance criteria

- [ ] Cube position/size configurable before render
- [ ] Ray–AABB intersection (slab method)
- [ ] Correct face normals at hit point
- [ ] Implements shared `Hittable` trait

## Dependencies

- Blocks: RT-012, RT-013
- Blocked by: RT-002

## Notes

Axis-aligned is enough for mandatory requirements; rotated cube is optional.

## Branch

`ticket/RT-006-cube`
