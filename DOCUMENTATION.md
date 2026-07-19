# Architecture & concepts

This is the **team-facing** doc: how `rt` is built and why, for anyone picking up this codebase. It is not a usage guide — for "how do I run this / add an object / change the light," see [`docs/DOCUMENTATION.md`](docs/DOCUMENTATION.md) (the RT-015 auditor-facing doc), which this file links to rather than repeats.

## Contents

1. [What is ray tracing](#1-what-is-ray-tracing)
2. [High-level architecture / data flow](#2-high-level-architecture--data-flow)
3. [Project structure](#3-project-structure)
4. [Module-by-module walkthrough](#4-module-by-module-walkthrough)
5. [Ray tracing math in this codebase](#5-ray-tracing-math-in-this-codebase)
6. [Rust concepts used in this codebase](#6-rust-concepts-used-in-this-codebase)
7. [Texture system internals](#7-texture-system-internals)
8. [See also](#8-see-also)

---

## 1. What is ray tracing

**Rasterization** (what most real-time graphics/games use) works forward from the geometry: take every triangle in the scene, project its vertices onto the 2D screen, and fill in the pixels it covers. It's fast and GPU-friendly, but it has no innate concept of "what can this pixel see" beyond the triangle being drawn — things like shadows, reflections, and refraction aren't part of the model, so renderers bolt them on with approximations (shadow maps, screen-space reflections, cubemaps).

**Ray tracing** works backward from the camera instead: for every pixel, shoot a ray from the eye *through* that pixel *into* the scene, and ask "what does this ray hit first?" Whatever it hits, at whatever point, is what that pixel shows. Because the ray is a real 3D line being tested against real 3D geometry, effects like shadows and reflections aren't special cases — they fall directly out of tracing more rays from the hit point.

This project's model has three kinds of rays, and they're all the *same* `Ray` type (`src/ray.rs`) traced the *same* way:

- **Primary ray** — camera → pixel → scene. One per pixel, built by `Camera::ray_through_pixel`.
- **Shadow ray** — hit point → light. Doesn't need a color, just a yes/no: does anything block it before the light? (`Scene::is_occluded`, called from `light::shade_lambertian`.)
- **Secondary rays** — reflection and refraction. A mirror's color depends on whatever *it* sees, which might itself be another mirror, or glass, or a shadowed surface — so a secondary ray is traced exactly like a primary ray, just starting from the hit point instead of the eye, and it can spawn its own shadow rays and its own secondary rays.

That last point is why **recursion** matters: `tracer::trace_recursive` calls *itself* to resolve a reflection or refraction, because the only way to know what a mirror shows is to trace another ray and see what *that* ray hits — which might, in turn, need its own reflection traced. Nothing bounds this naturally (two facing mirrors would recurse forever), so this codebase caps it with `TraceOptions::max_depth` (§5).

---

## 2. High-level architecture / data flow

This is the actual path one pixel takes, function by function:

```
main.rs                                  parse_args() -> Args { scene, width, height, -r/-R/-t, max_depth }
   │
   ├─ scenes::sceneN_xxx(aspect) ───────► (Scene, Camera)
   │                                       Scene { objects: Vec<Object>, lights, ambient, background }
   │                                       Camera: eye/look-at/FOV → orthonormal (right, up, forward) basis
   │
   ├─ texture::set_textures_enabled(args.textures)      process-wide switch, see §7
   │
   └─ tracer::render_frame_with(&scene, &camera, w, h, &TraceOptions)
          │
          │   for y in 0..height { for x in 0..width {    row-major, y=0 = top row (PPM scan order)
          │
          ├──► camera.ray_through_pixel(x, y, w, h) ───► Ray { origin: eye, direction }   (camera.rs)
          │
          └──► tracer::trace_recursive(scene, ray, opts, depth=0)
                  │
                  ├─► scene.hit(ray, 0.001, ∞)                closest-hit scan over Vec<Object>  (scene.rs)
                  │      └─► Sphere|Plane|Cube|Cylinder::hit(ray, t_min, t_max)     (objects/*.rs)
                  │             └─► Option<HitRecord { t, point, normal, front_face, material, uv }>
                  │
                  ├─► light::shade_lambertian(&hit, &lights, ambient, occluded) ──► `local: Color`
                  │      ├─ texture::sample_albedo(&hit.material, hit.uv)   ◄── §7 chokepoint
                  │      └─ per light: shadow ray ──► scene.is_occluded(...)
                  │
                  ├─ depth >= max_depth?                                 return `local`
                  │
                  ├─ refractions on && material is dielectric?
                  │      trace_recursive for the reflected AND transmitted rays,
                  │      Schlick-blend them, tint by sample_albedo(&hit.material, hit.uv)
                  │
                  ├─ reflections on && material.reflectivity > 0?
                  │      trace_recursive for the mirror bounce,
                  │      blend `local * (1-k) + reflected * sample_albedo(...) * k`
                  │
                  └─ (else) `local`
                  │
                  └─► Color  ──► pushed into pixels: Vec<Color>
          }}
   │
   └─► ppm::write_ppm_p3(&mut out, w, h, &pixels)  ──► "P3\nW H\n255\nR G B\n..." → file or stdout
```

Two things worth calling out because they're easy to miss on a first read:

- **`trace_recursive` is the one function that does closest-hit *and* shading *and* bounce.** `shade_lambertian` never sees reflection/refraction — it only computes the local (direct-light) color. The recursive bounce logic lives entirely in `tracer.rs`, wrapped *around* that local color.
- **Every recursive call re-does the closest-hit scan from scratch** (`scene.hit(ray, 0.001, f64::INFINITY)` at the top of `trace_recursive`). A reflected ray doesn't remember anything about the surface it bounced off of — it's a brand new ray traced exactly like a primary ray. This is *why* textures on mirrors work correctly (§7): each bounce resolves its own `HitRecord`, with its own `uv`, independently.

---

## 3. Project structure

```
.
├── Cargo.toml / Cargo.lock     Rust package manifest — zero external dependencies (stdlib only)
├── LICENSE                     MIT license
├── README.md                   Project front page / entry point
├── DOCUMENTATION.md            This file — architecture & concepts for the team
├── TEAM.md                     Contributors, focus areas, ticket ownership
│
├── src/
│   ├── main.rs                 CLI entry point: arg parsing, scene dispatch, render, write output
│   ├── vec3.rs                 Vec3 (position/direction) and Color (RGB) — the shared math types
│   ├── ray.rs                  Ray { origin, direction }
│   ├── camera.rs                Pinhole camera: eye/look-at/FOV → per-pixel primary rays
│   ├── material.rs              Material: albedo, reflectivity, ior, texture_path
│   ├── light.rs                 Light enum, Lambertian shading, shadow rays
│   ├── objects/
│   │   ├── mod.rs               HitRecord, the Hittable trait, shared hit constructor
│   │   ├── sphere.rs             Sphere: quadratic intersection + spherical UV
│   │   ├── plane.rs              Plane: Hessian-form intersection + tiled UV
│   │   ├── cube.rs               Cube: AABB slab-method intersection + per-face UV
│   │   └── cylinder.rs           Cylinder: side + caps intersection + angle/height UV
│   ├── scene.rs                 Scene container (objects/lights/background), closest-hit + occlusion
│   ├── tracer.rs                Recursive trace loop: shading + reflection + refraction
│   ├── texture.rs               P3 PPM texture loader, cache, sample_albedo chokepoint
│   ├── ppm.rs                   P3 PPM writer
│   └── scenes.rs                Named scene builders — the 4 audit scenes + bonus demos
│
├── docs/
│   ├── DOCUMENTATION.md         Auditor-facing usage guide: features, CLI, code examples (RT-015)
│   ├── BRANCHING.md             Branch naming / PR workflow convention
│   └── GITHUB_GITEA_SYNC.md     Notes on the GitHub → Gitea mirror
│
├── scenes/
│   ├── README.md                What each rendered image should look like + its regen command
│   ├── *.ppm                    Rendered audit + bonus-demo images (the actual deliverables)
│   └── pngs/*.png               Same renders as PNG, for quick viewing outside a PPM-aware tool
│
├── scripts/
│   ├── setup-ticket-branches.sh One git branch per ticket
│   ├── gen-demo-textures.py      Generates the procedural P3 PPM textures under textures/
│   └── ppm_to_png.py             Converts scenes/*.ppm to scenes/pngs/*.png
│
├── textures/
│   └── *.ppm                    Demo P3 PPM textures sampled via Material.texture_path
│
└── tickets/
    ├── ticket-tracker.md         Full tracker: every ticket, deps, acceptance, coverage matrix
    ├── BOARD.md                  Live sprint board
    ├── DEPENDENCIES.md           Ticket dependency graph
    ├── template.md                Template for new ticket files
    └── RT-001 … RT-018.md         One file per ticket
```

---

## 4. Module-by-module walkthrough

### `vec3.rs`

Two types everything else is built on: `Vec3 { x, y, z }` (positions and directions) and `Color { r, g, b }` (never `f32` anywhere in this project — everything is `f64`). Both are `#[derive(Clone, Copy, Debug, PartialEq)]`.

`Vec3` has the operations you'd expect: `dot`, `cross`, `length`/`length_squared`, `normalize`, plus operator overloads (`Add`, `Sub`, `Neg`, `Mul<f64>`, `Div<f64>`, and `Mul<Vec3> for f64` so `2.0 * v` and `v * 2.0` both compile). `Color` overloads `Mul for Color` specifically as a **component-wise** multiply — `self.r * rhs.r, self.g * rhs.g, self.b * rhs.b` — because that's what "tint albedo by light color" or "tint a mirror by its texture" actually means; it is *not* a dot product. `Color::to_rgb8()` clamps to `[0,1]` and rounds to `u8` for the PPM writer.

### `ray.rs`

```rust
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction: direction.normalize() }
    }
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
```

`Ray::new` always normalizes the direction, so nothing downstream (intersection math, `t` values) has to worry about a ray with a non-unit direction. `at(t)` is the parametric point along the ray — used everywhere a "hit point" is computed (`ray.at(root)` in `sphere.rs`, `ray.at(t)` in `plane.rs`/`cube.rs`, etc.).

### `camera.rs`

`Camera::look_at(eye, look_at, world_up, vfov_degrees, aspect_ratio)` builds an orthonormal basis once (`orthonormal_frame`, a cross-product construction with a fallback axis when looking nearly parallel to `world_up`) and caches `half_height`/`half_width` (derived from `tan(vfov/2)`) so every subsequent `get_ray` call is just a couple of multiplies and adds — no trig per pixel.

```rust
pub fn get_ray(self, u: f64, v: f64) -> Ray {
    let ndc_x = 2.0 * u - 1.0;
    let ndc_y = 2.0 * v - 1.0;
    let direction =
        self.forward + self.right * (ndc_x * self.half_width) + self.up * (ndc_y * self.half_height);
    Ray::new(self.eye, direction)
}
```

`ray_through_pixel(x, y, width, height)` is the bridge from integer pixel coordinates to the normalized `(u, v) ∈ [0,1]²` this expects, flipping `v` (`1.0 - (y+0.5)/height`) so that image row `y = 0` — the *first* row written to the PPM — corresponds to the *top* of the frame.

### `material.rs`

```rust
pub struct Material {
    pub albedo: Color,
    pub reflectivity: f64,        // 0..1, mirror blend (RT-016)
    pub ior: f64,                 // >1.0 enables Snell's-law refraction (RT-017)
    pub texture_path: Option<&'static str>,  // RT-018
}
```

One struct covers all three bonus features — a material is diffuse by default (`Material::solid`), and `metal`/`glass`/`textured` are just constructors that set different fields on the same struct. `with_texture` is a builder method so features compose (`Material::metal(..).with_texture(..)` is a textured mirror — used in `scene_texture_reflection_demo`).

### `light.rs`

`Light` is a two-variant enum (`Point { position, color, intensity }`, `Directional { direction, color, intensity }`). `Light::sample(hit_point) -> Option<(Vec3, f64)>` returns the direction *toward* the light and the max shadow-ray distance (`f64::INFINITY` for directional lights, since there's no "reaching" a directional light).

`shade_lambertian` is the direct-lighting function — no bounce logic here, that's `tracer.rs`'s job:

```rust
pub fn shade_lambertian(
    hit: &HitRecord,
    lights: &[Light],
    ambient: f64,
    mut occluded: impl FnMut(&Ray, f64) -> bool,
) -> Color {
    let albedo = sample_albedo(&hit.material, hit.uv);
    let mut color = albedo * ambient.max(0.0);
    for light in lights {
        // ... skip lights with intensity <= 0, skip back-facing (ndotl <= 0),
        // cast a shadow ray, skip if occluded, else:
        color += albedo * light.color() * (intensity * ndotl);
    }
    color.clamp()
}
```

Note it takes an occlusion **closure**, not a `Scene` — see §6.

### `objects/mod.rs`

The shared contract every primitive implements:

```rust
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,      // faces against the incoming ray
    pub front_face: bool,
    pub material: Material,
    pub uv: (f64, f64),    // RT-018
}
```

`HitRecord::from_outward_normal(t, point, outward_normal, ray, material, uv)` is called by all four shapes' `hit()` — it takes the shape's raw *geometric* (always-outward) normal and flips it if needed so `normal` always faces back toward the ray, recording whether the ray hit the front or back of the surface (`front_face`). This is what lets `tracer.rs` pick the right `ior` ratio when a refracted ray is exiting glass instead of entering it.

### `objects/sphere.rs`

`Sphere { center, radius, material }`. Intersection is the classic ray-sphere quadratic, solved in "half-b" form to avoid a stray factor of 2:

```rust
let oc = ray.origin - self.center;
let a = ray.direction.length_squared();
let half_b = oc.dot(ray.direction);
let c = oc.length_squared() - self.radius * self.radius;
let discriminant = half_b * half_b - a * c;
```

A negative discriminant is a miss. Otherwise it takes the *nearer* root that falls inside `[t_min, t_max]`, falling back to the farther root if the near one is out of range (this is what makes "camera positioned inside a sphere" still resolve to a sensible hit — used by the from-inside tests). UV comes from the already-computed outward normal (see §5/§7).

### `objects/plane.rs`

Stored in Hessian normal form (`normal · x + offset == 0`, unit `normal`) rather than as a point — `Plane::ground(y, material)` and `Plane::from_point_normal(point, normal, material)` both funnel into this representation. Intersection is a single linear solve for `t`; a ray parallel to the plane (`normal.dot(ray.direction) ≈ 0`) is a miss. Also carries `tile_size: f64` (default `2.0`) purely for RT-018 UV tiling (§7) — everything about the *intersection* math is unchanged from before textures existed.

### `objects/cube.rs`

Axis-aligned box stored as `min`/`max` corners, intersected with the **slab method**: treat the box as the intersection of three axis-aligned "slabs" (pairs of parallel planes), compute each slab's entry/exit `t` via the inverted ray direction, and narrow a running `[t_enter, t_exit]` interval across all three axes. If that interval ever goes empty, it's a miss. Whichever slab produced the tightest `t_enter`/`t_exit` also supplies that face's normal, which `face_uv` (RT-018) uses to pick which two axes become `u`/`v`.

### `objects/cylinder.rs`

`Cylinder { base, radius, height, material }` — axis is always world `+Y`; `base` is the center of the *bottom* disk. `hit()` tries three candidate surfaces and keeps the closest: the side wall (`hit_side`, an infinite-tube quadratic in the XZ plane, clipped to `[y_min, y_max]`) and the two caps (`hit_cap`, a plane intersection at fixed `y`, restricted to points within `radius` of the axis). All three candidates are funneled through one `consider` closure (see §6) so "closest so far" only needs to live in one place.

### `scene.rs`

`Object` is a hand-rolled sum type for the four shapes:

```rust
pub enum Object {
    Sphere(Sphere), Plane(Plane), Cube(Cube), Cylinder(Cylinder),
}
```

...and `impl Hittable for Object` just matches and delegates. `Scene { objects: Vec<Object>, lights: Vec<Light>, ambient: f64, background: Background }` is the world: `Scene::hit` is a linear closest-hit scan (shrinking `t_max` to the closest `t` found so far as it goes, so later objects can only produce a *closer* hit), and `Scene::is_occluded` is a cheaper `.any()` scan for shadow rays — it doesn't care *which* object blocks the light, only *whether* one does. `Background` (`Solid(Color)` or a `Sky { zenith, horizon }` vertical gradient) is what a primary or secondary ray sees on a total miss.

### `tracer.rs`

The recursive core — see §2's data-flow diagram for the full shape of `trace_recursive`. Also home to the standalone geometry functions used by the bounce logic:

```rust
pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 { /* mirror formula, §5 */ }
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Option<Vec3> { /* Snell's law, §5 */ }
pub fn schlick(cosine: f64, etai_over_etat: f64) -> f64 { /* Fresnel approx, §5 */ }
```

`TraceOptions { reflections, refractions, max_depth }` is what `-r`/`-R`/`--max-depth` map onto — when both flags are `false` (the default), every material shades as flat Lambertian diffuse regardless of what `reflectivity`/`ior` it carries, which keeps the four mandatory audit scenes fast.

### `texture.rs`

Covered in depth in §7 — briefly: parses P3 PPM into an in-memory `Texture { width, height, pixels: Vec<Color> }`, caches decoded textures behind a path-keyed `HashMap`, and exposes the one function (`sample_albedo`) that the rest of the renderer calls to resolve a material's color.

### `ppm.rs`

The output side. `write_ppm_p3(out, width, height, pixels: &[Color])` writes the `P3` header (`P3` / `width height` / `255`) then one `R G B` line per pixel, asserting the buffer length matches `width * height` first. Pixel order is row-major, top-to-bottom — the same order `render_frame`/`render_frame_with` build the buffer in, and the same order `Camera::ray_through_pixel` assumes for its `y`-flip.

### `scenes.rs`

Named builders, each returning `(Scene, Camera)` (or, for the world shared between Scene 3 and Scene 4, a bare `Scene` via `scene3_world()`, paired with `scene3_camera_front`/`scene3_camera_alt`). Every builder follows the same shape: a `// --- configurable ---` block of local `let`s for every tunable number, then a `Scene::new()...add(...).add_light(...)` chain. This is also where texture paths get attached to specific scene objects (e.g. `scene3_world`'s cube and cylinder carry `.with_texture(...)` — see §7 for why that's safe without `-t`).

### `main.rs`

The only module that knows about `std::env`/`std::process`/file I/O. `parse_args` is a small hand-rolled flag parser (no CLI-parsing crate — this project has zero dependencies) producing an `Args` struct; `parse_scene` maps `--scene` values (numeric or named, e.g. `3`/`all`, `9`/`texture-reflection`) to a `SceneId` enum, which `main()` then matches to pick both the scene-builder function to call and a human-readable label for the status line. `main()` is also the *only* call site of `texture::set_textures_enabled` (§7) and the thing that decides file-vs-stdout output.

---

## 5. Ray tracing math in this codebase

**Camera ray generation.** `Camera::look_at` turns `vfov_degrees` into `half_height = tan(vfov/2)` (at unit focal distance) and `half_width = half_height * aspect`. `get_ray(u, v)` remaps normalized image coordinates `u, v ∈ [0,1]` to `[-1, 1]` (NDC) and builds the direction as `forward + right*ndc_x*half_width + up*ndc_y*half_height` — i.e. walk out from the look direction along the camera's own right/up axes, scaled by how far off-center this pixel is and by the field of view.

**Sphere intersection** is the textbook `|O + tD − C|² = r²` quadratic, expanded into `at² + bt + c = 0` and solved with the quadratic formula (in half-`b` form, see `sphere.rs` above). A negative discriminant means the ray's line never touches the sphere at all.

**Plane intersection** substitutes the ray equation into the plane's Hessian form and solves directly for a single `t` — no quadratic, since a line meets a plane at most once (or never, if parallel, or always, if the ray lies *in* the plane — treated as a miss here for simplicity).

**Cube (AABB) intersection** is the **slab method**: a box is the intersection of three infinite slabs (one pair of parallel planes per axis). Compute where the ray enters and exits each slab, intersect those three `t`-intervals, and whatever's left (if non-empty) is where the ray is inside the box.

**Cylinder intersection** splits into the side wall — an infinite-tube quadratic that only involves `x`/`z` (the axis is `+Y`, so `y` never enters the tube equation), clipped afterward to the finite height range — and the two caps, which are just plane intersections at fixed `y`, restricted to the disk of radius `r`.

**Lambertian shading + shadow rays.** Diffuse brightness is proportional to `cos θ` between the surface normal and the direction to the light (`hit.normal.dot(to_light)`, clamped so a light behind the surface contributes nothing). Before counting a light, a **shadow ray** is cast from the (bias-offset) hit point toward it; if anything blocks that ray before the light's own distance, the light contributes nothing to this point — this is the entire shadow algorithm, no shadow maps involved.

**Reflection formula.** `reflect(v, n) = v − 2(v·n)n` — the standard mirror-about-the-normal formula (`tracer::reflect`).

**Refraction / Snell's law + Fresnel.** `tracer::refract` implements the vector form of Snell's law: split the refracted direction into a component perpendicular to the normal (scaled by the ratio of refractive indices, `etai_over_etat`) and a component parallel to it (derived from whatever length is "left over" so the result stays unit length); if that leftover would be negative, the ray can't refract at all — **total internal reflection**. `tracer::schlick` is the Schlick approximation of the Fresnel equations: how much light reflects vs. transmits at a given angle, used in `trace_recursive` to *blend* the reflected and refracted colors rather than picking one or the other.

**Why bounce depth is capped.** Reflection and refraction are both implemented as recursive calls to `trace_recursive`. Nothing about the geometry guarantees that recursion terminates — a fully mirrored sphere reflecting a fully mirrored plane would bounce forever. `TraceOptions.max_depth` (`--max-depth`, default `5`) is checked at the top of every recursive call; once `depth >= max_depth`, the function returns the local (unbounced) shaded color instead of recursing further.

---

## 6. Rust concepts used in this codebase

Aimed at teammates newer to Rust — each concept explained through this project's own code, not a generic example.

### Ownership and `Copy` types

`Vec3`, `Color`, `Ray`, `Material`, `HitRecord`, `Camera`, and each shape (`Sphere`, `Plane`, `Cube`, `Cylinder`) are all `#[derive(Clone, Copy, ...)]`. `Copy` means passing one of these by value doesn't "move" (invalidate) the original — you can use it in an expression and keep using it afterward, without an explicit `.clone()`. That matters constantly in math-heavy code like `sphere.rs`'s `hit()`:

```rust
let oc = ray.origin - self.center;
let a = ray.direction.length_squared();   // ray.direction read again below
let half_b = oc.dot(ray.direction);       // ...and again here
```

`ray` gets read multiple times across `hit()`, and is *also* passed into `HitRecord::from_outward_normal(..., ray, ...)` at the end — none of that needs `.clone()` because `Ray` is `Copy`. Rust only allows `#[derive(Copy)]` on types that are cheap to duplicate (no heap allocation, no destructor) — three `f64`s (24 bytes) trivially qualifies; a `Vec<Object>` (as on `Scene`) does not, which is why `Scene` derives only `Clone`.

### Traits: the `Hittable` pattern

```rust
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
```

`Sphere`, `Plane`, `Cube`, `Cylinder`, and `Object` (the enum wrapping all four) each `impl Hittable`. This is Rust's version of an interface — but notice this codebase never uses `dyn Hittable` (a trait object / heap-allocated, dynamically-dispatched value). Instead, `Object::hit` just pattern-matches to the concrete shape and calls its `hit` directly:

```rust
impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Self::Sphere(o) => o.hit(ray, t_min, t_max),
            Self::Plane(o) => o.hit(ray, t_min, t_max),
            Self::Cube(o) => o.hit(ray, t_min, t_max),
            Self::Cylinder(o) => o.hit(ray, t_min, t_max),
        }
    }
}
```

`scene.rs`'s own doc comment explains the tradeoff: "enum keeps the world heap-free and object-safe-free" — `Scene::objects: Vec<Object>` is a flat, stack-value-per-element vector with no indirection, at the cost of the object list only ever being able to hold these four specific shapes (adding a fifth shape means adding an `Object` variant, not just a new type that happens to implement `Hittable`).

### Enums + pattern matching

Three enums carry most of the scene model: `Object` (above), `Background` (`Solid(Color)` vs. `Sky { zenith, horizon }`), and `Light` (`Point { .. }` vs. `Directional { .. }`). `Light::intensity` shows matching multiple variants that share a field name in one arm, using `|` (or-pattern) and `..` (ignore the rest):

```rust
pub fn intensity(self) -> f64 {
    match self {
        Self::Point { intensity, .. } | Self::Directional { intensity, .. } => intensity,
    }
}
```

### `Option<T>`

`Material.texture_path: Option<&'static str>` — `None` means "no texture, always use `albedo`"; `Some(path)` means "sample this file." `texture::sample_albedo` shows Rust's `let-else` pattern for unwrapping with an early return:

```rust
let Some(path) = material.texture_path else {
    return material.albedo;
};
```

`&'static str` (rather than `String`) is what keeps `Material` — and everything containing it (`Sphere`, `HitRecord`, ...) — `Copy`: a `String` owns a heap buffer and can't be `Copy`, but a `&'static str` is just a pointer + length into memory that lives for the whole program (in practice, string literals written directly in `scenes.rs`, e.g. `"textures/checker_red.ppm"`).

### `Result<T, E>` error handling

`texture::load_ppm_p3(path: &str) -> Result<Texture, String>` is the clearest example:

```rust
let text = fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?;
```

`fs::read_to_string` returns `Result<String, std::io::Error>`; `.map_err(...)` translates that into this function's own `String` error type, and `?` either unwraps the `Ok` value or returns the `Err` immediately from `load_ppm_p3`. Critically, the *caller* (`texture::get_texture`) never lets that `Err` propagate further — it's caught, turned into one `eprintln!` warning, and cached as a `None` entry so a missing/broken texture file degrades to the material's solid `albedo` instead of crashing the renderer.

### Closures / higher-order functions

`light::shade_lambertian` takes its occlusion test as a closure parameter, not a `Scene`:

```rust
pub fn shade_lambertian(
    hit: &HitRecord,
    lights: &[Light],
    ambient: f64,
    mut occluded: impl FnMut(&Ray, f64) -> bool,
) -> Color
```

called from `tracer.rs` as:

```rust
shade_lambertian(&hit, &scene.lights, scene.ambient, |shadow, t_max| {
    scene.is_occluded(shadow, t_max)
})
```

`light.rs` never needs to know what a `Scene` even is — it just needs "a thing I can call with a ray and a max distance that tells me if I'm blocked." This decouples the shading math from however occlusion actually gets computed. `Cylinder::hit` has a second, different closure flavor: `hit_side`/`hit_cap` both take `consider: &mut dyn FnMut(f64, Vec3)` — a **trait object** (dynamically dispatched, since it's a `dyn`) so multiple methods can share and mutate the same "closest hit so far" state through one closure, without `Cylinder::hit` having to duplicate that bookkeeping per candidate surface.

### Modules and visibility

Rust items are private to their module by default; `pub` opts in. `Vec3`'s fields are all `pub` — there's no invariant to protect on a plain 3-tuple of numbers. `Camera`'s fields are **not** `pub` (no keyword at all) — `eye`, `forward`, `right`, `up`, `half_height`, `half_width` are only reachable through `Camera::look_at` (which builds a *valid, orthonormal* basis) and read-only getters like `pub fn eye(self) -> Vec3`, so external code can't hand it a `right`/`up` pair that isn't actually perpendicular. `texture.rs` also uses `pub(crate)` — visible anywhere in this crate, not exported beyond it — for `TEST_ENABLE_LOCK`, a synchronization primitive `tracer.rs`'s test module needs to borrow from `texture.rs`'s tests (see below) without it being a real public API.

### Why `texture.rs` uses `Arc`/`Mutex`/`OnceLock`/`AtomicBool` — in a single-threaded renderer

`render_frame_with`'s pixel loop is a plain sequential `for y { for x { ... } }` — there's no `std::thread::spawn` anywhere in this crate, and the shipped `rt` binary never runs more than one thread. So why does `texture.rs` reach for thread-safety primitives? Two independent reasons, both explained in the module's own comments:

1. **`static` items must be `Sync`.** The global texture cache and the `-t` on/off switch have to live somewhere that isn't tied to any one function call — a `static`. But a plain `bool`/`HashMap` can't be mutated behind a `static` in safe Rust (statics are immutable by default); `AtomicBool` and `Mutex<HashMap<...>>` are the standard-library-sanctioned ways to get *interior mutability* on a `static` without `unsafe`. `OnceLock` is what lazily builds the `HashMap` on first use (the stdlib's built-in answer to what a `lazy_static!`-style crate used to be needed for). None of this is really "for concurrency" here — it's the price of safe global mutable state in Rust, concurrency or not.
2. **`cargo test` genuinely *is* multithreaded**, even though the `rt` binary isn't — the test harness runs `#[test]` functions on a thread pool by default. `TEXTURES_ENABLED` is process-wide, so two tests toggling it concurrently could race; that's what `TEST_ENABLE_LOCK` (a plain `Mutex<()>`, held for the duration of any test that flips the flag) exists to prevent. `Arc<Texture>` (rather than `Rc<Texture>`) specifically because a `static` must be `Send + Sync`, and `Rc`'s non-atomic reference count is neither — `Arc`'s atomic count is the only one allowed to live in a `static`, even in a program that never actually contends on it at runtime.

### The `#[cfg(test)] mod tests` convention

Every one of the 16 source files ends with a block like:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    // ...
}
```

`#[cfg(test)]` means this module only exists when compiling for `cargo test` — it adds nothing to `cargo build --release`. `use super::*` pulls everything from the enclosing (real) module into scope, including private items — so, for example, `cube.rs`'s tests can exercise `face_uv`/`axis_frac` directly even though neither is `pub`. Tests live next to the code they test rather than in a separate `tests/` directory, which keeps a change to (say) `sphere_uv`'s formula and the test that pins its expected values in the same file, one scroll apart.

---

## 7. Texture system internals

(Brief — for how to *use* textures, UV formulas per shape, and the `-t` flag, see [`docs/DOCUMENTATION.md`](docs/DOCUMENTATION.md)'s "Textures (bonus RT-018)" section.)

**UV threading.** Each shape computes its own `(u, v)` right where it already knows the hit geometry, inside its own `hit()`: `sphere.rs`'s `sphere_uv(outward)`, `plane.rs`'s `Plane::uv_at(point)`, `cube.rs`'s `face_uv(point, outward, min, max)`, `cylinder.rs`'s `Cylinder::uv_at(point)`. Every one of these feeds straight into `HitRecord::from_outward_normal(..., uv)`, so by the time shading code sees a `HitRecord`, `uv` is already resolved — nothing downstream needs to know *which* shape it came from.

**`sample_albedo` is the single chokepoint.** `light::shade_lambertian` and both bounce sites in `tracer::trace_recursive` (the metal-reflection tint and the dielectric reflect/refract tint) call `texture::sample_albedo(&hit.material, hit.uv)` instead of reading `hit.material.albedo` directly. Because each recursive bounce in `trace_recursive` re-runs `scene.hit(...)` from scratch and gets its *own* fresh `HitRecord` (§2), a reflected or refracted ray always resolves the texture color at *its own* hit point — never a cached value from the surface it bounced off of. That's what makes a textured mirror show the correct checker cell in its own reflection rather than a flat/default color.

---

## 8. See also

- [`docs/DOCUMENTATION.md`](docs/DOCUMENTATION.md) — usage / auditor guide: CLI flags, code examples for adding objects and lights, how to reproduce every audit image.
- [`tickets/ticket-tracker.md`](tickets/ticket-tracker.md) — project history: every ticket (RT-001–RT-018), dependencies, and the deliverable coverage matrix.
- [`scenes/README.md`](scenes/README.md) — what each rendered image should look like, and the exact command to regenerate it.
