# netrunner_cli — task runner
# Install just:      cargo install just
# Install git-cliff: cargo install git-cliff
# Install vhs:       brew install vhs  OR  go install github.com/charmbracelet/vhs@latest
# Install nushell:   cargo install nu
# Usage: just <task>

# ── Default ───────────────────────────────────────────────────────────────────

default:
    @just --list

# ── Tool checks ───────────────────────────────────────────────────────────────

_check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { \
        echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; \
    }

_check-vhs:
    @command -v vhs >/dev/null 2>&1 || { \
        echo "❌ vhs not found."; \
        echo "   macOS:  brew install vhs"; \
        echo "   Any:    go install github.com/charmbracelet/vhs@latest"; \
        exit 1; \
    }

# Install all recommended development tools
install-tools:
    @echo "Installing development tools…"
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff
    @echo "✅ All tools installed!"

# ── Build ─────────────────────────────────────────────────────────────────────

# Build in debug mode
build:
    cargo build

# Build optimised release binary
build-release:
    cargo build --release

# ── Run ───────────────────────────────────────────────────────────────────────

run:
    cargo run

# ── Test ──────────────────────────────────────────────────────────────────────

test:
    cargo test

# ── Code quality ──────────────────────────────────────────────────────────────

# Check without building
check:
    cargo check

# Format all code
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt --check

# Run clippy — warnings are errors, deprecations are allowed
clippy:
    cargo clippy --all-targets --all-features -- -D warnings -A deprecated

# Run all quality checks (fmt, clippy, test) — must pass before a release
check-all: fmt-check clippy test
    @echo "✅ All checks passed!"

# ── Clean / install ───────────────────────────────────────────────────────────

clean:
    cargo clean

install:
    cargo install --path .

# ── Documentation ─────────────────────────────────────────────────────────────

doc:
    cargo doc --no-deps --open

# ── Changelog ─────────────────────────────────────────────────────────────────

# Regenerate the full CHANGELOG.md from all tags
changelog: _check-git-cliff
    @echo "Generating full changelog…"
    git-cliff --output CHANGELOG.md
    @echo "✅ CHANGELOG.md updated."

# Prepend only unreleased commits to CHANGELOG.md
changelog-unreleased: _check-git-cliff
    git-cliff --unreleased --prepend CHANGELOG.md
    @echo "✅ Unreleased changes prepended."

# Preview changelog for the next release without writing the file
changelog-preview: _check-git-cliff
    @git-cliff --unreleased

# Generate changelog for a specific version tag
changelog-version version: _check-git-cliff
    @echo "Generating changelog for v{{version}}…"
    git-cliff --tag v{{version}} --output CHANGELOG.md
    @echo "✅ Changelog generated for v{{version}}."

# Generate changelog for the latest tag only
changelog-latest: _check-git-cliff
    @echo "Generating changelog for latest tag…"
    git-cliff --latest --output CHANGELOG.md
    @echo "✅ Latest changelog generated."

# ── Version bump ──────────────────────────────────────────────────────────────
# Usage: just bump 0.6.0
#
# Flow:
#   1. check-all  — fmt-check → clippy → tests  (quality gate)
#   2. bump_version.nu — updates Cargo.toml, Cargo.lock, CHANGELOG.md, commits, tags
#
# After this completes push with:
#   just release <version>          (GitHub only)
#   just release-all <version>      (GitHub + Gitea)

bump version: check-all _check-git-cliff
    @echo "Bumping version to {{version}}…"
    @nu scripts/bump_version.nu {{version}}

# ── Publish (crates.io) ───────────────────────────────────────────────────────

# Dry-run publish — smoke-test packaging without uploading
publish-dry: check-all
    cargo publish --dry-run

# Full publish — runs the quality gate first
publish: check-all
    cargo publish

# Run the pre-publish readiness check
check-publish:
    nu scripts/check_publish.nu

# ── Dependencies ──────────────────────────────────────────────────────────────

# Update all dependencies (Cargo.lock only)
update:
    cargo update

# Show outdated dependencies (requires cargo-outdated)
outdated:
    cargo outdated

# Upgrade all dependency pins, run quality gate, commit and push (via Nu script)
upgrade-deps:
    nu scripts/upgrade_deps.nu

# Upgrade deps in dry-run mode (show plan without committing)
upgrade-deps-dry:
    nu scripts/upgrade_deps.nu --dry-run

# Legacy alias — same as upgrade-deps
update-deps: upgrade-deps

# ── VHS Demo GIFs ─────────────────────────────────────────────────────────────

VHS_DIR       := "examples/vhs"
VHS_GENERATED := "examples/vhs/target"

# Build release binaries then record all tapes in examples/vhs/
vhs-all: _check-vhs
    @echo "🔨 Building release binaries…"
    cargo build --release
    cargo build --release --example statistics_dashboard
    @mkdir -p {{VHS_GENERATED}}
    @echo "╔════════════════════════════════════════════╗"
    @echo "║   Recording all VHS tapes                 ║"
    @echo "╚════════════════════════════════════════════╝"
    @for tape in {{VHS_DIR}}/*.tape; do \
        echo "▶  $tape"; \
        vhs "$tape" || echo "❌ Failed: $tape"; \
    done
    @echo "✅ All GIFs → {{VHS_GENERATED}}/"

# Pull GIF files from Git LFS (run once after a fresh clone)
lfs-pull:
    @command -v git-lfs >/dev/null 2>&1 || { \
        echo "❌ git-lfs not found. Install with: brew install git-lfs"; exit 1; \
    }
    git lfs pull
    @echo "✅ LFS objects pulled."

# ── Utility ───────────────────────────────────────────────────────────────────

# Show current version
version:
    @nu scripts/version.nu

# Show project info
info:
    @echo "Project: Netrunner CLI"
    @echo "Version: $(just version)"
    @echo "Author:  Sorin Albu-Irimies"
    @echo "License: MIT"

# View changelog
view-changelog:
    @cat CHANGELOG.md

# Show configured git remotes
remotes:
    @echo "Configured git remotes:"
    @git remote -v

# ── Git helpers ───────────────────────────────────────────────────────────────

# Stage everything and commit
commit message:
    git add .
    git commit -m "{{message}}"

# Push main branch to origin
push:
    git push origin main

# Push all tags to origin
push-tags:
    git push --tags

# ── Release workflows ─────────────────────────────────────────────────────────

# Bump version then push to GitHub
release version: (bump version)
    @echo "Pushing to GitHub…"
    git push origin main
    git push origin v{{version}}
    @echo "✅ Release v{{version}} complete on GitHub!"

# Bump version then push to Gitea
release-gitea version: (bump version)
    @echo "Pushing to Gitea…"
    git push gitea main
    git push gitea v{{version}}
    @echo "✅ Release v{{version}} complete on Gitea!"

# Bump version then push to both GitHub and Gitea
release-all version: (bump version)
    @echo "Pushing to both GitHub and Gitea…"
    git push origin main
    git push gitea main
    git push origin v{{version}}
    git push gitea v{{version}}
    @echo "✅ Release v{{version}} complete on both remotes!"

# ── Gitea ─────────────────────────────────────────────────────────────────────

push-gitea:
    git push gitea main

push-all:
    git push origin main
    git push gitea main
    @echo "✅ Pushed to both GitHub and Gitea!"

push-tags-all:
    git push origin --tags
    git push gitea --tags
    @echo "✅ Tags pushed to both remotes!"

push-release-all:
    @echo "Pushing release to both remotes…"
    git push origin main
    git push gitea main
    git push origin --tags
    git push gitea --tags
    @echo "✅ Release pushed to both remotes!"

sync-gitea:
    @echo "Syncing Gitea with GitHub…"
    git push gitea main --force
    git push gitea --tags --force
    @echo "✅ Gitea synced!"

setup-gitea url:
    nu scripts/setup_gitea.nu {{url}}
