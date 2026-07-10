# RT-018: Textures (bonus)

| Field | Value |
|-------|-------|
| **Status** | Backlog |
| **Assignee** | @iana |
| **Priority** | P2 |
| **Epic** | bonus |

## Description

Map 2D textures onto object surfaces (e.g. UV on sphere or cube faces).

## Acceptance criteria

- [ ] Load or embed at least one texture image
- [ ] Sample texture at hit point for albedo color
- [ ] Behind CLI flag (e.g. `-t` / `--textures`)
- [ ] Document texture paths and supported formats

## Dependencies

- Blocks: —
- Blocked by: RT-004–007, RT-009

## Notes

Project spec suggests `-t` flag; keep off by default for speed.
