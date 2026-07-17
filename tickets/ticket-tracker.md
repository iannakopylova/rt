> Legend: 🔴 Blocked · 🟡 To Do · 🟢 In Progress · 🔵 In Review · ✅ Done · ⬜ Backlog
>
> **Sources**: individual tickets (`RT-*.md`) · [BOARD.md](./BOARD.md) · [DEPENDENCIES.md](./DEPENDENCIES.md) · [TEAM.md](../TEAM.md)

---

# rt Ticket Tracker

Last refreshed: 2026-07-17 (RT-011 Scene 1 done)

> **Board vs tracker**: [BOARD.md](./BOARD.md) is the live sprint board (who is on what). This file is the full requirements-style tracker: every ticket, deps, acceptance summary, and coverage by epic.

---

## 1) Scope Contract

This tracker covers the mandatory ray tracer deliverable and optional bonuses.

Execution order (see [DEPENDENCIES.md](./DEPENDENCIES.md)):

1. Bootstrap & math (`RT-001`, `RT-002`)
2. Parallel core — camera + object primitives (`RT-003`–`RT-007`)
3. Tracer integration (`RT-009`)
4. Shading & PPM I/O (`RT-008`, `RT-010`)
5. Four audit scenes (`RT-011`–`RT-014`)
6. Auditor documentation (`RT-015`)
7. Bonus materials / textures (`RT-016`–`RT-018`, optional)

This repository delivers a Rust ray tracer that writes **P3 PPM** images (stdlib-first; no heavy graphics crates unless the team agrees).

---

## 2) Team Assignment

| Person | Handle | Focus Area | Tickets |
|--------|--------|------------|---------|
| **Iana** | `@iana` | Foundation, tracer core, PPM | RT-001, RT-002, RT-009, RT-010, RT-018 |
| **Sofia** | `@sofia` | Camera + object primitives | RT-003, RT-004, RT-005, RT-006, RT-007, RT-017 |
| **Andriana** | `@andriana` | Lighting, scenes, docs | RT-008, RT-011, RT-012, RT-013, RT-014, RT-015, RT-016 |

| Person | Active ticket | Next up |
|--------|---------------|---------|
| **Iana** | — (PPM done) | RT-018 bonus / help scenes |
| **Sofia** | — (objects done) | RT-017 bonus after core |
| **Andriana** | RT-012 | RT-013 after Scene 2 |

---

## 3) Epics and Deliverable IDs

### Epics

| Epic | Scope |
|------|--------|
| `foundation` | Cargo setup, math, rays, camera |
| `objects` | Sphere, cube, plane, cylinder |
| `rendering` | Intersection, shading, shadows, PPM |
| `scenes` | Four required 800×600 images |
| `docs` | Markdown documentation for auditors |
| `bonus` | Reflection, refraction, textures |

### Mandatory deliverables (audit checklist)

| ID | Deliverable | Tickets |
|----|-------------|---------|
| D1 | Cargo project builds & runs | RT-001 |
| D2 | Vec3 / Ray / Color math | RT-002 |
| D3 | Configurable camera (pos, angle, FOV) | RT-003 |
| D4 | Sphere, plane, cube, cylinder primitives | RT-004–007 |
| D5 | Ray–object intersection + closest hit | RT-009 |
| D6 | Lights, brightness, shadows | RT-008 |
| D7 | P3 PPM output + resolution control | RT-010 |
| D8 | Scene 1 — sphere only (800×600) | RT-011 |
| D9 | Scene 2 — plane + cube, dimmer light | RT-012 |
| D10 | Scene 3 — all four objects | RT-013 |
| D11 | Scene 4 — same scene, alternate camera | RT-014 |
| D12 | Auditor markdown docs + examples | RT-015 |

### Bonus deliverables

| ID | Deliverable | Tickets |
|----|-------------|---------|
| B1 | Reflection (CLI flag) | RT-016 |
| B2 | Refraction (CLI flag) | RT-017 |
| B3 | Textures (CLI flag) | RT-018 |

---

## Phase 0 — Bootstrap

> **Goal**: Initialize the Cargo project and module layout.

| ID | Status | Ticket | Size | Deps | Coverage | Assignee |
|----|--------|--------|------|------|----------|----------|
| RT-001 | ✅ | **Cargo project & repo structure**: `Cargo.toml`, module stubs under `src/` + `objects/`, placeholder `cargo run`, README. | S | — | D1 | @iana |

---

## Phase 1 — Foundation Math

> **Goal**: Shared types used by camera, objects, and tracer.

| ID | Status | Ticket | Size | Deps | Coverage | Assignee |
|----|--------|--------|------|------|----------|----------|
| RT-002 | ✅ | **Vec3, Ray, Color**: vector ops, ray origin/direction, RGB clamp + `u8` conversion, unit tests. | M | RT-001 | D2 | @iana |

---

## Phase 2 — Parallel Core (camera + objects)

> **Goal**: Unblock tracer work; up to 3 people in parallel after RT-002.

| ID | Status | Ticket | Size | Deps | Coverage | Assignee |
|----|--------|--------|------|------|----------|----------|
| RT-003 | ✅ | **Camera**: position, look-at, up, FOV; `get_ray(u, v) -> Ray`; document Scene 4 camera change. | M | RT-002 | D3 | @sofia |
| RT-004 | ✅ | **Sphere**: center + radius; ray–sphere hit; `Hittable` trait; visible in minimal scene. | M | RT-002 | D4 | @sofia |
| RT-005 | ✅ | **Plane**: point + normal; ray–plane hit; ground for Scenes 2–3. | M | RT-002 | D4 | @sofia |
| RT-006 | ✅ | **Cube**: AABB (min/max or center+size); slab intersection; face normals. | M | RT-002 | D4 | @sofia |
| RT-007 | ✅ | **Cylinder**: position, radius, height; side (+ caps if finite); normals. | M | RT-002 | D4 | @sofia |

---

## Phase 3 — Tracer Integration

> **Goal**: Closest-hit pipeline that objects and lights plug into.

| ID | Status | Ticket | Size | Deps | Coverage | Assignee |
|----|--------|--------|------|------|----------|----------|
| RT-009 | ✅ | **Ray tracer core**: `Hittable` + `HitRecord`; scene object list; `trace` / `render_frame`; solid or sky background. | L | RT-002, RT-003, ≥1 object | D5 | @iana |

---

## Phase 4 — Shading & I/O

> **Goal**: Lit images written as valid PPM.

| ID | Status | Ticket | Size | Deps | Coverage | Assignee |
|----|--------|--------|------|------|----------|----------|
| RT-008 | 🟢 | **Lights, brightness & shadows**: configurable intensity; Lambertian; shadow rays; Scene 2 dimmer than Scene 1. | L | RT-009, RT-004–007 | D6 | @andriana |
| RT-010 | ✅ | **PPM (P3) output**: `P3` header + RGB body; `cargo run > out.ppm`; `--width` / `--height` / `--output`. | M | RT-009 | D7 | @iana |

---

## Phase 5 — Audit Scenes

> **Goal**: Four reproducible **800×600** PPM images.

| ID | Status | Ticket | Size | Deps | Coverage | Assignee |
|----|--------|--------|------|------|----------|----------|
| RT-011 | ✅ | **Scene 1 — sphere only**: one sphere, lighting/shadows, 800×600 PPM. | M | RT-004, RT-008, RT-010 | D8 | @andriana |
| RT-012 | 🟡 | **Scene 2 — plane + cube**: dimmer light than Scene 1, shadows, 800×600 PPM. | M | RT-005, RT-006, RT-008, RT-010 | D9 | @andriana |
| RT-013 | 🟡 | **Scene 3 — all four objects**: cube + sphere + cylinder + plane, 800×600 PPM. | M | RT-004–007, RT-008, RT-010 | D10 | @andriana |
| RT-014 | 🟡 | **Scene 4 — alt camera**: same layout as Scene 3, different camera, 800×600 PPM. | S | RT-003, RT-013 | D11 | @andriana |

---

## Phase 6 — Documentation

> **Goal**: Auditor can build scenes and reproduce images without guessing.

| ID | Status | Ticket | Size | Deps | Coverage | Assignee |
|----|--------|--------|------|------|----------|----------|
| RT-015 | 🟡 | **Auditor documentation**: features; examples (objects, brightness, camera); PPM/resolution; reproduce all 4 scenes. Outline can start anytime. | M | RT-010–014 (finalize) | D12 | @andriana |

---

## Phase 7 — Bonus (optional)

> **Goal**: Extra credit features behind CLI flags.

| ID | Status | Ticket | Size | Deps | Coverage | Assignee |
|----|--------|--------|------|------|----------|----------|
| RT-016 | ⬜ | **Reflection**: reflective materials; max bounce depth; CLI flag (e.g. `-r`). | M | RT-008, RT-009 | B1 | @andriana |
| RT-017 | ⬜ | **Refraction**: Snell's law / glass; CLI flag; demo object. | L | RT-009, RT-016 (recommended) | B2 | @sofia |
| RT-018 | ⬜ | **Textures**: UV sample albedo; CLI flag (e.g. `-t`); document paths/formats. | M | RT-004–007, RT-009 | B3 | @iana |

---

## 4) Critical Path

```
RT-001 → RT-002 → RT-004 → RT-009 → RT-008 → RT-010 → RT-013 → RT-014 → RT-015
```

With 3 people: run **RT-003–007** in parallel after RT-002, then **RT-008 + RT-010** after RT-009.

Full dependency graph: [DEPENDENCIES.md](./DEPENDENCIES.md).

---

## 5) Deliverable Coverage Matrix

| Deliverable | Description | Tickets | Status |
|-------------|-------------|---------|--------|
| D1 | Project builds & runs | RT-001 | ✅ |
| D2 | Math primitives | RT-002 | ✅ |
| D3 | Camera | RT-003 | ✅ |
| D4 | Four primitives | RT-004 ✅, RT-005 ✅, RT-006 ✅, RT-007 ✅ | ✅ |
| D5 | Tracer core / closest hit | RT-009 | ✅ |
| D6 | Lights & shadows | RT-008 | 🟢 |
| D7 | PPM output | RT-010 | ✅ |
| D8 | Scene 1 | RT-011 | ✅ |
| D9 | Scene 2 | RT-012 | 🟡 |
| D10 | Scene 3 | RT-013 | 🟡 |
| D11 | Scene 4 | RT-014 | 🟡 |
| D12 | Documentation | RT-015 | 🟡 |
| B1 | Reflection | RT-016 | ⬜ |
| B2 | Refraction | RT-017 | ⬜ |
| B3 | Textures | RT-018 | ⬜ |

---

## 6) Immediate Next Work Queue

1. **Andriana** — Scene 2 **RT-012** (dimmer light), then **RT-013 → RT-014**.
2. **Iana** — PPM done; can help scenes or start bonus **RT-018**.
3. **Sofia** — objects complete; next is bonus **RT-017** after lighting.
4. Finalize **RT-015** once scenes land.

---

## Summary by Person

### Iana — 5 tickets

| Phase | Tickets |
|-------|---------|
| 0–1 | RT-001 ✅, RT-002 ✅ |
| 3–4 | RT-009 ✅, RT-010 ✅ |
| 7 | RT-018 ⬜ |

### Sofia — 6 tickets

| Phase | Tickets |
|-------|---------|
| 2 | RT-003 ✅, RT-004 ✅, RT-005 ✅, RT-006 ✅, RT-007 ✅ |
| 7 | RT-017 ⬜ |

### Andriana — 7 tickets

| Phase | Tickets |
|-------|---------|
| 4–6 | RT-008 🟢, RT-011 ✅, RT-012 🟡, RT-013 🟡, RT-014 🟡, RT-015 🟡 |
| 7 | RT-016 ⬜ |

---

## Total: 18 tickets

| Status | Count |
|--------|-------|
| ✅ Done | 10 |
| 🟢 In Progress | 1 |
| 🟡 To Do | 4 |
| ⬜ Backlog | 3 |
| 🔵 In Review | 0 |

| Priority | Count |
|----------|-------|
| P0 | 3 (RT-001, RT-002, RT-003) |
| P1 | 12 (mandatory remaining) |
| P2 | 3 (bonus) |

| Size (estimate) | Count |
|-----------------|-------|
| S | 2 |
| M | 13 |
| L | 3 |

---

## How to update this file

1. Change status in the ticket file (`RT-XXX-*.md`) **and** [BOARD.md](./BOARD.md).
2. Mirror the status emoji in the phase tables above.
3. Bump **Last refreshed** at the top.
4. When a deliverable’s tickets are all ✅, mark its row ✅ in the coverage matrix.
