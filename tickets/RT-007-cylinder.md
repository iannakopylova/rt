# RT-007: Cylinder primitive

| Field | Value |
|-------|-------|
| **Status** | To Do |
| **Assignee** | @sofia |
| **Priority** | P1 |
| **Epic** | objects |

## Description

Implement a finite or infinite cylinder (document which). Typical choice: vertical cylinder with center, radius, and height on Y axis.

## Acceptance criteria

- [ ] Cylinder position, radius, height configurable
- [ ] Ray–cylinder intersection with cap handling if finite
- [ ] Correct normals on side and caps
- [ ] Implements shared `Hittable` trait

## Dependencies

- Blocks: RT-013
- Blocked by: RT-002

## Notes

Look up cylinder ray intersection; cap disks are often a separate check.
