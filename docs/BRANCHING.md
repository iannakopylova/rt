# Branching & pull requests

GitHub is the source of truth. Every ticket gets its own branch and PR so teammates can review.

## Branch name

```
ticket/RT-XXX-short-title
```

Examples:

- `ticket/RT-003-camera`
- `ticket/RT-009-ray-tracer-core`

## Workflow

```bash
# 1. Start from up-to-date main
git checkout main
git pull origin main

# 2. Create your ticket branch
git checkout -b ticket/RT-00X-short-title

# 3. Implement the ticket, commit often
git add .
git commit -m "RT-00X: short description of why"

# 4. Push and open / update the PR
git push -u origin HEAD
# If a draft PR already exists for this ticket, just push.
# Otherwise:
gh pr create --fill
```

## Pull requests

- One PR **per ticket** (do not mix tickets).
- Title: `RT-00X: Short title`
- Mark **Draft** while working; mark **Ready for review** when done.
- Move the ticket to **In Review** on [`tickets/BOARD.md`](../tickets/BOARD.md) when you request review.
- After merge: move ticket to **Done**, delete the branch.

## Reviewers

| Author | Ask review from |
|--------|-----------------|
| Iana | Sofia or Andriana |
| Sofia | Iana or Andriana |
| Andriana | Iana or Sofia |

## Already on `main`

**RT-001** and **RT-002** landed in the initial commit. Review them via the GitHub issues linked from the board (no separate PR).
