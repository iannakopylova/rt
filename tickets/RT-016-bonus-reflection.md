# RT-016: Reflection (bonus)

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @andriana |
| **Priority** | P2 |
| **Epic** | bonus |

## Description

Add reflective materials: recursive reflection rays with a max depth limit.

## Acceptance criteria

- [x] Materials can be marked reflective (e.g. metal) → `Material::metal`
- [x] Reflection visible in at least one demo image → `scenes/scene_reflection.ppm` / `--scene reflection -r`
- [x] Behind CLI flag (e.g. `-r` / `--reflection`) for performance
- [x] Max bounce depth configurable → `--max-depth N`

## Dependencies

- Blocks: —
- Blocked by: RT-009, RT-008

## Notes

Example reference image in project brief shows reflection.
