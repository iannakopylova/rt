# rt

A ray tracer written from scratch in Rust — the Zone01/01-edu **`rt`** project. It renders 3D scenes (spheres, planes, cubes, cylinders) to **P3 PPM** images via real ray/object intersection, Lambertian lighting with shadows, and recursive reflection/refraction. No graphics or CLI-parsing crates — `Cargo.toml` has zero dependencies.

## Features

- **Four primitives**: sphere, infinite plane, axis-aligned cube, finite cylinder
- **Configurable pinhole camera**: eye position, look-at target, world up, vertical FOV
- **Lighting & shadows**: point and directional lights, Lambertian diffuse shading, shadow rays, configurable brightness/ambient
- **P3 PPM output**: `--width`/`--height`/`--output`, or stream to stdout
- **Bonus — reflection** (`-r`/`--reflection`): recursive mirror bounces, configurable max depth
- **Bonus — refraction** (`-R`/`--refraction`): Snell's law + Fresnel (Schlick), total internal reflection
- **Bonus — textures** (`-t`/`--textures`): P3 PPM albedo maps, per-object UV mapping, works through reflection and refraction

## Quick start

```bash
# Build
cargo build --release

# Render an audit scene (all four object types)
cargo run --release -- --scene 3 --width 800 --height 600 -o scenes/scene3_all.ppm

# See every flag
cargo run --release -- --help
```

Defaults are a small dev preview (400×300, scene 1); use `--width 800 --height 600` for audit-quality renders. Without `--output`, a P3 PPM streams to stdout.

## Project structure

```
src/          Rust source — math, camera, objects/, scene, tracer, textures, PPM I/O, CLI
docs/         Auditor-facing usage guide, branching, and Gitea sync notes
scenes/       Rendered audit + bonus-demo images (.ppm), with .png previews and a README
scripts/      Ticket-branch setup, demo texture generator, PPM→PNG converter
textures/     Demo P3 PPM textures sampled by materials
tickets/      Ticket tracker, sprint board, one file per ticket
```

For the full annotated file tree and a module-by-module breakdown of how the renderer actually works, see **[`DOCUMENTATION.md`](DOCUMENTATION.md)**.

## Documentation

| File | What it's for |
|------|----------------|
| [`docs/DOCUMENTATION.md`](docs/DOCUMENTATION.md) | **Usage guide** — features, CLI flags, code examples for adding objects/lights/textures, how to reproduce every audit image |
| [`DOCUMENTATION.md`](DOCUMENTATION.md) | **Architecture guide** — ray tracing theory, data flow, module-by-module walkthrough, the math, and the Rust concepts behind the code |
| [`scenes/README.md`](scenes/README.md) | **Example renders** — what each image should look like, plus its exact regenerate command |

## Repo workflow

Push to GitHub only; Gitea (Zone01) mirrors automatically — see [`docs/GITHUB_GITEA_SYNC.md`](docs/GITHUB_GITEA_SYNC.md). Use `ticket/RT-XXX-short-title` branches when starting a ticket — see [`docs/BRANCHING.md`](docs/BRANCHING.md).

## Authors

- 🗂️ Iana Kopylova - [ikopylov](https://discordapp.com/users/1279339146833297509)
- 👩‍💻 Sofia Busho - [sbusho](https://discordapp.com/users/1276592724979613697)
- ✍️ Adriana Stas - [astas](https://discordapp.com/users/780150798927134740)


## License

MIT — see [`LICENSE`](LICENSE).
