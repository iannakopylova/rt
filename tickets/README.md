# Ticket Tracker

Lightweight issue tracker for the **rt** ray tracer project (3-person team).

## Quick start

1. Open **[BOARD.md](./BOARD.md)** for the current sprint view.
2. Check **[DEPENDENCIES.md](./DEPENDENCIES.md)** to see what must be done before your ticket.
3. Pick a ticket from **To Do** and assign yourself in the ticket file.
4. Move it to **In Progress** on the board.
5. When done, open a PR (or merge) and move the ticket to **Done**.

## Ticket IDs

Format: `RT-###` (e.g. `RT-007`). Increment the number for new tickets.

## Creating a new ticket

```bash
cp tickets/template.md tickets/RT-0XX-short-title.md
```

Fill in the template, then add a row to [BOARD.md](./BOARD.md).

## Status workflow

```
Backlog → To Do → In Progress → In Review → Done
```

| Status        | Meaning                                      |
|---------------|----------------------------------------------|
| **Backlog**   | Planned but not ready to start               |
| **To Do**     | Ready to be picked up                        |
| **In Progress** | Someone is actively working on it          |
| **In Review** | Code/docs done; awaiting peer review         |
| **Done**      | Merged and accepted                          |

## Priority

- **P0** — Blocks other work or mandatory for audit
- **P1** — Required for submission
- **P2** — Nice to have / bonus
- **P3** — Future / optional

## Epics

| Epic          | Scope                                      |
|---------------|--------------------------------------------|
| `foundation`  | Cargo setup, math, rays, camera            |
| `objects`     | Sphere, cube, plane, cylinder              |
| `rendering`   | Intersection, shading, shadows, PPM        |
| `scenes`      | Four required 800×600 images               |
| `docs`        | Markdown documentation for auditors        |
| `bonus`       | Reflection, refraction, textures, etc.     |

## Team

See [../TEAM.md](../TEAM.md) for assignees (**Iana**, **Sofia**, **Andriana**).

**Current:** RT-001 & RT-002 done (@iana). **Sofia** can start RT-003 / RT-004. **Iana** next: RT-009.
