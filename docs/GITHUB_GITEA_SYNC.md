# GitHub → Gitea sync (Zone01)

**GitHub is the source of truth.** Clone, pull, and push only to GitHub. Gitea is updated automatically by GitHub Actions.

## One-time setup

### 1. Create repo on GitHub

Create an empty repo on GitHub with the **same name** as Gitea: `rt`  
(Do not add README, .gitignore, or license — this repo already has them.)

Example: `https://github.com/YOUR_USERNAME/rt`

### 2. Create repo on Gitea (Zone01)

If not already created:

`https://platform.zone01.gr/git/YOUR_USERNAME/rt`

Existing Gitea repo for this project: `https://platform.zone01.gr/git/ikopylov/rt`

### 3. Gitea access token

1. Gitea → avatar (top right) → **Settings**
2. **Applications** (or **Security → Access Tokens**)
3. **Generate New Token**
   - Name: e.g. `github-sync-rt`
   - Scopes: enable at least **repo** (read/write)
4. Copy the token immediately (shown only once)

### 4. GitHub repository secrets

Open your GitHub repo → **Settings** → **Secrets and variables** → **Actions** → **New repository secret**

| Secret name       | Value |
|-------------------|-------|
| `GITEA_TOKEN`     | Token from step 3 |
| `GITEA_REPO_URL`  | `https://platform.zone01.gr/git/ikopylov/rt` |
| `GITEA_USERNAME`  | Your Gitea username (e.g. `ikopylov`) |

### 5. Point local `origin` to GitHub

```bash
# If origin still points to Gitea, replace it:
git remote remove origin
git remote add origin https://github.com/YOUR_USERNAME/rt.git

git push -u origin main
```

After the first push, the **Sync to Gitea** workflow runs and mirrors branches/tags to Gitea.

## Daily workflow

```bash
git pull origin main
# ... work ...
git add .
git commit -m "your message"
git push origin main
```

- Do **not** push code directly to Gitea — the next GitHub sync uses `--force` and will overwrite it.
- Sync usually completes within 1–2 minutes after each GitHub push.
- Issues and PRs stay on GitHub; only branches, commits, and tags sync to Gitea.

## Manual sync

GitHub → **Actions** → **Sync to Gitea** → **Run workflow**
