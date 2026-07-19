# RT-018: Textures (bonus)

| Field | Value |
|-------|-------|
| **Status** | Done |
| **Assignee** | @iana |
| **Priority** | P2 |
| **Epic** | bonus |

## Description

Map 2D textures onto object surfaces (e.g. UV on sphere or cube faces).

## Acceptance criteria

- [x] Load or embed at least one texture image → P3 PPM loader in `src/texture.rs`; 5 demo textures under `textures/` (`scripts/gen-demo-textures.py`)
- [x] Sample texture at hit point for albedo color → `texture::sample_albedo(material, uv)`, wired into `light::shade_lambertian` and both reflection/refraction tints in `tracer::trace_recursive`
- [x] Behind CLI flag (e.g. `-t` / `--textures`) → `-t` / `--textures`, off by default
- [x] Document texture paths and supported formats → `docs/DOCUMENTATION.md` § Textures (bonus RT-018)

## Dependencies

- Blocks: —
- Blocked by: RT-004–007, RT-009

## Notes

UV mapping lives per `Hittable`: sphere (spherical: `u = 0.5 + atan2(z,x)/2π`,
`v = 0.5 - asin(y/r)/π`), plane (tiled world XZ via a new `tile_size` field,
default `2.0`), cube (per-face planar off the hit normal's dominant axis),
cylinder (`u` = angle around `+Y`, `v` = height fraction). `HitRecord` gained a
`uv: (f64, f64)` field — `f64` not `f32`, to match this project's
`f64`-everywhere convention (ticket allows adjusting argument types) —
populated by each primitive's own `hit()`.

`sample_albedo`'s signature is locked to `(material, uv)`, leaving no room for
a `TraceOptions`-style flag, so `-t` enablement is a process-wide `AtomicBool`
(`texture::set_textures_enabled`, called once from `main`) consulted inside
`sample_albedo` itself. The tracer is single-threaded so this is safe, and it
guarantees `-t`-off behavior is exactly `material.albedo` regardless of
whether a `texture_path` is set — verified byte-for-byte: `--scene 3` with
`-t` omitted reproduces the pre-existing `scenes/scene3_all.ppm` exactly
(`cmp` / `md5sum` match) even though its cube and cylinder now carry a
`texture_path`. Same for `--scene 4` (shares `scene3_world()`).

Demo scenes: `--scene texture-sphere` (`7`), `texture-plane` (`8`), and
`texture-reflection` (`9`, textured **and** mirrored — confirms RT-016
reflection tints by the sampled color at the reflection's own hit point, not
a flat/cached one; verified visually in `scenes/scene_texture_reflection.ppm`).
Scene 3's cube + cylinder are also textured (sphere/plane stay solid) so
`scenes/scene_texture_all.ppm` exercises mixed materials across all four
object types in one image.
