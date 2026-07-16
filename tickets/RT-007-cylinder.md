# RT-007: Cylinder primitive

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @sofia |
| **Priority** | P1 |
| **Epic** | objects |

## Description

Implement a finite or infinite cylinder (document which). Typical choice: vertical cylinder with center, radius, and height on Y axis.

## Acceptance criteria

- [x] Cylinder position, radius, height configurable
- [x] Ray–cylinder intersection with cap handling if finite
- [x] Correct normals on side and caps
- [x] Implements shared `Hittable` trait

## Dependencies

- Blocks: RT-013
- Blocked by: RT-002

## Notes

Look up cylinder ray intersection; cap disks are often a separate check.

Implemented in `src/objects/cylinder.rs`:
- **Finite** Y-aligned cylinder (documented)
- Stored as bottom `base` + `radius` + `height` (`from_midpoint` helper)
- Side wall via half-b XZ quadratic; disk caps; face-oriented normals
