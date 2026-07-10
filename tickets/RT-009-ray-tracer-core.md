# RT-009: Ray tracer core loop

| Field | Value |
|-------|-------|
| **Status** | To Do |
| **Assignee** | @iana |
| **Priority** | P1 |
| **Epic** | rendering |

## Description

Implement the main tracing pipeline: for each pixel, cast camera ray, find closest intersection among all objects, compute shaded color.

## Acceptance criteria

- [ ] `Hittable` trait: `hit(ray, t_min, t_max) -> Option<HitRecord>`
- [ ] Scene holds a list of objects + lights
- [ ] `trace(ray) -> Color` with closest-hit selection
- [ ] Background color when no hit (e.g. sky gradient or solid)

## Dependencies

- Blocks: RT-008, RT-010, all object tickets
- Blocked by: RT-002, RT-003, at least one object

## Notes

This is the integration point — coordinate with object owners on the trait API early.
