# RT-008: Lights, brightness & shadows

| Field | Value |
|-------|-------|
| **Status** | To Do |
| **Assignee** | @andriana |
| **Priority** | P1 |
| **Epic** | rendering |

## Description

Add at least one point or directional light with adjustable brightness. Implement shadow rays: if a light is occluded between hit point and light, pixel is in shadow.

## Acceptance criteria

- [ ] Light intensity/brightness configurable per scene
- [ ] Simple diffuse (Lambertian) shading
- [ ] Shadow rays with small offset to avoid self-intersection
- [ ] Scene 2 uses **lower brightness** than Scene 1

## Dependencies

- Blocks: RT-011, RT-012, RT-013, RT-014
- Blocked by: RT-004–007, RT-009

## Notes

Ambient term (small constant) helps avoid pure-black shadows.

## Branch

`ticket/RT-008-lighting-shadows`
