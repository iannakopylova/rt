# RT-008: Lights, brightness & shadows

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @andriana |
| **Priority** | P1 |
| **Epic** | rendering |

## Description

Add at least one point or directional light with adjustable brightness. Implement shadow rays: if a light is occluded between hit point and light, pixel is in shadow.

## Acceptance criteria

- [x] Light intensity/brightness configurable per scene
- [x] Simple diffuse (Lambertian) shading
- [x] Shadow rays with small offset to avoid self-intersection
- [x] Scene 2 uses **lower brightness** than Scene 1
  (`SCENE2_LIGHT_INTENSITY` < `SCENE1_LIGHT_INTENSITY`; helpers `Light::scene1_key` / `Light::scene2_key`)

## Dependencies

- Blocks: RT-011, RT-012, RT-013, RT-014
- Blocked by: RT-004–007, RT-009

## Notes

Ambient term (small constant) helps avoid pure-black shadows.
