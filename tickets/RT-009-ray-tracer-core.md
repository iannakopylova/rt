# RT-009: Ray tracer core loop

| Field | Value |
|-------|-------|
| **Status** | In Progress |
| **Assignee** | @iana |
| **Priority** | P1 |
| **Epic** | rendering |

## Description

Implement the main tracing pipeline: for each pixel, cast camera ray, find closest intersection among all objects, compute shaded color.

## Acceptance criteria

- [x] `Hittable` trait: `hit(ray, t_min, t_max) -> Option<HitRecord>`
- [x] Scene holds a list of objects + lights
- [x] `trace(ray) -> Color` with closest-hit selection
- [x] Background color when no hit (e.g. sky gradient or solid)

## Notes

Filled alongside RT-008 so lighting can cast shadow rays. This is the integration point — Iana can still refine (sky gradient, API polish) before marking Done.

## Dependencies

- Blocks: RT-008, RT-010, all object tickets
- Blocked by: RT-002, RT-003, at least one object
