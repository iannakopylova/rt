#!/usr/bin/env bash
set -euo pipefail

RT="$(find /mnt/c/Users/ianna/OneDrive -maxdepth 2 -type d -name rt | head -1)"
cd "$RT"
echo "Working in: $PWD"

git checkout main
git pull origin main

git add docs/BRANCHING.md README.md tickets/README.md tickets/BOARD.md
if ! git diff --cached --quiet; then
  git commit -m "Add branch-per-ticket workflow for team reviews"
  git push origin main
else
  echo "No doc changes to commit"
fi

# Open review issues for work already on main
ISSUE1=$(gh issue create \
  --title "Review RT-001: Cargo project & repo structure" \
  --body "$(cat <<'EOF'
## Summary
RT-001 is already on \`main\` (initial commit). Please review the project layout.

## Checklist
- [ ] \`Cargo.toml\` / \`cargo build\` / \`cargo run\`
- [ ] Module skeleton under \`src/\` and \`src/objects/\`
- [ ] README is clear

## Files to check
- \`Cargo.toml\`
- \`src/main.rs\`
- \`src/**\` stubs
- \`README.md\`

Assignee: @iannakopylova — reviewers: Sofia, Andriana
EOF
)")
echo "ISSUE1=$ISSUE1"

ISSUE2=$(gh issue create \
  --title "Review RT-002: Vec3, Ray, Color types" \
  --body "$(cat <<'EOF'
## Summary
RT-002 is already on \`main\`. Please review the math primitives.

## Checklist
- [ ] \`Vec3\` ops: add, sub, mul, dot, cross, length, normalize
- [ ] \`Ray\` with normalized direction + \`at(t)\`
- [ ] \`Color\` clamp + \`to_rgb8\`
- [ ] Unit tests pass (\`cargo test\`)

## Files to check
- \`src/vec3.rs\`
- \`src/ray.rs\`

Assignee: @iannakopylova — reviewers: Sofia, Andriana
EOF
)")
echo "ISSUE2=$ISSUE2"

# ticket_id|slug|title|assignee|draft_or_ready
TICKETS=(
  "RT-003|camera|Camera (position, angle, FOV)|sofia|draft"
  "RT-004|sphere|Sphere primitive|sofia|draft"
  "RT-005|plane|Flat plane primitive|sofia|draft"
  "RT-006|cube|Cube primitive|sofia|draft"
  "RT-007|cylinder|Cylinder primitive|sofia|draft"
  "RT-008|lighting-shadows|Lights, brightness & shadows|andriana|draft"
  "RT-009|ray-tracer-core|Ray tracer core loop|iana|draft"
  "RT-010|ppm-output|PPM (P3) output & resolution flag|iana|draft"
  "RT-011|scene-sphere|Scene 1 — sphere only|andriana|draft"
  "RT-012|scene-plane-cube|Scene 2 — plane + cube|andriana|draft"
  "RT-013|scene-all-objects|Scene 3 — all four objects|andriana|draft"
  "RT-014|scene-alt-camera|Scene 4 — alt camera|andriana|draft"
  "RT-015|documentation|Auditor documentation|andriana|draft"
  "RT-016|bonus-reflection|Reflection (bonus)|andriana|draft"
  "RT-017|bonus-refraction|Refraction (bonus)|sofia|draft"
  "RT-018|bonus-textures|Textures (bonus)|iana|draft"
)

mkdir -p /tmp/rt-pr-urls
: > /tmp/rt-pr-urls/list.txt

for entry in "${TICKETS[@]}"; do
  IFS='|' read -r ID SLUG TITLE ASSIGNEE MODE <<< "$entry"
  BRANCH="ticket/${ID}-${SLUG}"
  TICKET_FILE=$(ls tickets/${ID}-*.md | head -1)
  echo "==== Creating $BRANCH from $TICKET_FILE ===="

  git checkout main
  git checkout -B "$BRANCH"

  # Add branch field to ticket if missing
  if ! grep -q '^\*\*Branch\*\*' "$TICKET_FILE"; then
    # Insert Branch after Epic row in the table — append under Dependencies instead for simplicity
    printf '\n## Branch\n\n`%s`\n' "$BRANCH" >> "$TICKET_FILE"
  fi

  git add "$TICKET_FILE"
  git commit -m "${ID}: open ticket branch for review workflow" || true
  git push -u origin "$BRANCH" --force-with-lease

  BODY_FILE=$(mktemp)
  cat > "$BODY_FILE" <<EOF
## Summary
- Ticket **${ID}**: ${TITLE}
- Assignee: **@${ASSIGNEE}**
- Branch: \`${BRANCH}\`

## Ticket
See [\`${TICKET_FILE}\`](${TICKET_FILE}) for acceptance criteria.

## Test plan
- [ ] Acceptance criteria in the ticket file are met
- [ ] \`cargo build\` / \`cargo test\` pass (when code is added)
- [ ] Another teammate reviewed the PR

EOF

  if [ "$MODE" = "draft" ]; then
    URL=$(gh pr create --draft --base main --head "$BRANCH" \
      --title "${ID}: ${TITLE}" \
      --body-file "$BODY_FILE")
  else
    URL=$(gh pr create --base main --head "$BRANCH" \
      --title "${ID}: ${TITLE}" \
      --body-file "$BODY_FILE")
  fi
  echo "$ID $URL" | tee -a /tmp/rt-pr-urls/list.txt
  rm -f "$BODY_FILE"
done

git checkout main

echo "==== DONE ===="
echo "Review issues:"
echo "$ISSUE1"
echo "$ISSUE2"
echo "PRs:"
cat /tmp/rt-pr-urls/list.txt
