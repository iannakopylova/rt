# RT-010: PPM (P3) output & resolution flag

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @iana |
| **Priority** | P1 |
| **Epic** | rendering |

## Description

Write rendered image to stdout in P3 PPM format. Support easy resolution changes for dev vs final audit images.

## Acceptance criteria

- [x] Header: `P3`, `width height`, `255`
- [x] Body: one `R G B` line per pixel, top-left to bottom-right
- [x] `cargo run > output.ppm` produces a valid image
- [x] Width/height configurable (CLI flag or const at top of `main`) — e.g. `400×300` for tests, `800×600` for submission

## Dependencies

- Blocks: RT-011–014
- Blocked by: RT-009

## Notes

Flags: `--width` / `-w`, `--height`, `--output` / `-o` (optional file instead of stdout).
Defaults: 400×300. Audit: `cargo run -- --width 800 --height 600 -o scene.ppm`.
Progress goes to stderr so stdout stays a clean PPM stream.
