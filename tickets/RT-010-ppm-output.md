# RT-010: PPM (P3) output & resolution flag

| Field | Value |
|-------|-------|
| **Status** | To Do |
| **Assignee** | @iana |
| **Priority** | P1 |
| **Epic** | rendering |

## Description

Write rendered image to stdout in P3 PPM format. Support easy resolution changes for dev vs final audit images.

## Acceptance criteria

- [ ] Header: `P3`, `width height`, `255`
- [ ] Body: one `R G B` line per pixel, top-left to bottom-right
- [ ] `cargo run > output.ppm` produces a valid image
- [ ] Width/height configurable (CLI flag or const at top of `main`) — e.g. `400×300` for tests, `800×600` for submission

## Dependencies

- Blocks: RT-011–014
- Blocked by: RT-009

## Notes

Suggested flags: `--width`, `--height`, `--output` (optional file instead of stdout).

## Branch

`ticket/RT-010-ppm-output`
