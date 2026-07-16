# RT-006: Cube primitive

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @sofia |
| **Priority** | P1 |
| **Epic** | objects |

## Description

Implement an axis-aligned cube (AABB) with configurable min/max corners or center + size.

## Acceptance criteria

- [x] Cube position/size configurable before render
- [x] Ray–AABB intersection (slab method)
- [x] Correct face normals at hit point
- [x] Implements shared `Hittable` trait

## Dependencies

- Blocks: RT-012, RT-013
- Blocked by: RT-002

## Notes

Axis-aligned is enough for mandatory requirements; rotated cube is optional.

Implemented in `src/objects/cube.rs`:
- Stored as `min`/`max` corners
- `from_corners` + `from_center_extent` (full edge length)
- Slab intersection with face normals from the hit slab
