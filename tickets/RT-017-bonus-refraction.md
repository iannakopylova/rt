# RT-017: Refraction (bonus)

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @sofia |
| **Priority** | P2 |
| **Epic** | bonus |

## Description

Add transparent/refractive materials (glass) using Snell's law.

## Acceptance criteria

- [x] Refractive index on material → `Material::glass` / `ior`
- [x] Refraction + optional reflection at interfaces → Snell's law + Schlick Fresnel (+ TIR)
- [x] Behind CLI flag → `-R` / `--refraction`
- [x] Demo object (glass sphere) → `scenes/scene_refraction.ppm` / `--scene refraction -R`

## Dependencies

- Blocks: —
- Blocked by: RT-009, RT-016 (optional)

## Notes

Harder than reflection; pair with RT-016 if one person owns “advanced materials”.
