#!/usr/bin/env nu
# Bump the package version for netrunner_cli.
# Updates Cargo.toml, README.md badge, Cargo.lock, runs checks, generates
# CHANGELOG, commits, and creates an annotated git tag.
#
# Usage:  nu scripts/bump_version.nu <new_version> [--yes]
# Example: nu scripts/bump_version.nu 0.7.0
#          nu scripts/bump_version.nu 0.7.0 --yes

def main [
    new_version: string,  # Target version (e.g. 0.7.0 or 1.0.0-beta.1)
    --yes (-y),           # Skip confirmation prompt
] {
    let red    = (ansi red)
    let green  = (ansi green)
    let yellow = (ansi yellow)
    let cyan   = (ansi cyan)
    let reset  = (ansi reset)

    # ── Validate version format ───────────────────────────────────────────────
    let version_re = '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$'
    if not ($new_version =~ $version_re) {
        print $"($red)Error: invalid version format '($new_version)'($reset)"
        print "Version must be X.Y.Z or X.Y.Z-pre (e.g. 0.7.0, 1.0.0-beta.1)"
        exit 1
    }

    # ── Read current version ──────────────────────────────────────────────────
    let current_version = (open Cargo.toml | get package.version)
    let tag_name = $"v($new_version)"

    print $"($cyan)netrunner_cli version bump($reset)"
    print $"  Current : ($yellow)($current_version)($reset)"
    print $"  New     : ($green)($new_version)($reset)"
    print ""

    # ── Guard: already at this version ───────────────────────────────────────
    if $current_version == $new_version {
        print $"($yellow)Already at version ($new_version) — nothing to do.($reset)"
        exit 0
    }

    # ── Guard: local tag already exists ──────────────────────────────────────
    let tag_exists = (
        do { run-external "git" "tag" "--list" $tag_name } | str trim | is-not-empty
    )
    if $tag_exists {
        print $"($red)Error: tag ($tag_name) already exists locally.($reset)"
        print $"Delete it first with:  git tag -d ($tag_name)"
        exit 1
    }

    # ── Confirmation ──────────────────────────────────────────────────────────
    if not $yes {
        print $"Bump ($current_version) → ($new_version) and create tag ($tag_name)?"
        let answer = (input "Continue? [y/N] " | str trim | str downcase)
        if $answer != "y" and $answer != "yes" {
            print $"($yellow)Aborted.($reset)"
            exit 0
        }
    }

    print ""

    # ─────────────────────────────────────────────────────────────────────────
    # Step 1 — Update Cargo.toml
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[1/7]($reset) Updating Cargo.toml …"
    let cargo_toml = (open --raw Cargo.toml
        | str replace --regex '(?m)^version = "[^"]*"' $"version = \"($new_version)\"")
    $cargo_toml | save --force Cargo.toml
    print $"  ($green)✔ Cargo.toml updated($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Step 2 — Update README.md version badge
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[2/7]($reset) Updating README.md badge …"
    let readme = (open --raw README.md
        | str replace --all --regex 'version-[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?-blue' $"version-($new_version)-blue")
    $readme | save --force README.md
    print $"  ($green)✔ README.md badge updated($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Step 3 — Regenerate Cargo.lock
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[3/7]($reset) Regenerating Cargo.lock …"
    run-external "cargo" "generate-lockfile"
    print $"  ($green)✔ Cargo.lock updated($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Step 4 — cargo fmt
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[4/7]($reset) Running cargo fmt …"
    run-external "cargo" "fmt"
    print $"  ($green)✔ Formatting applied($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Step 5 — cargo clippy
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[5/7]($reset) Running cargo clippy …"
    let clippy = (
        do { run-external "cargo" "clippy" "--all-targets" "--all-features" "--" "-D" "warnings" "-A" "deprecated" }
        | complete
    )
    if $clippy.exit_code != 0 {
        print $"  ($red)✘ Clippy reported errors \(exit ($clippy.exit_code)\).($reset)"
        print "  Fix the issues above and re-run the script."
        exit 1
    }
    print $"  ($green)✔ Clippy passed($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Step 6 — cargo test
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[6/7]($reset) Running cargo test …"
    let tests = (
        do { run-external "cargo" "test" "--all-features" "--all-targets" }
        | complete
    )
    if $tests.exit_code != 0 {
        print $"  ($red)✘ Tests failed \(exit ($tests.exit_code)\).($reset)"
        print "  Fix the failures above and re-run the script."
        exit 1
    }
    print $"  ($green)✔ All tests passed($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Step 7 — CHANGELOG, commit, tag
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[7/7]($reset) Generating CHANGELOG, committing, and tagging …"

    # Generate CHANGELOG.md with git-cliff
    let cliff = (
        do { run-external "git-cliff" "--tag" $tag_name "-o" "CHANGELOG.md" }
        | complete
    )
    if $cliff.exit_code != 0 {
        print $"  ($yellow)⚠ git-cliff exited ($cliff.exit_code) — continuing without updated CHANGELOG.($reset)"
        print "  Install git-cliff with: cargo install git-cliff"
    } else {
        print $"  ($green)✔ CHANGELOG.md generated($reset)"
    }

    # Stage files
    run-external "git" "add" "Cargo.toml" "Cargo.lock" "README.md" "CHANGELOG.md"

    # Commit
    run-external "git" "commit" "-m" $"chore: bump version to ($new_version)"
    print $"  ($green)✔ Commit created($reset)"

    # Annotated tag
    run-external "git" "tag" "-a" $tag_name "-m" $"Release ($tag_name)"
    print $"  ($green)✔ Tag ($tag_name) created($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Summary
    # ─────────────────────────────────────────────────────────────────────────
    print ""
    print $"($green)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print $"($green)  Version bump to ($new_version) complete! 🚀($reset)"
    print $"($green)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print ""
    print $"($cyan)Next steps:($reset)"
    print $"  Push commits and tag to GitHub:"
    print $"    git push origin main"
    print $"    git push origin ($tag_name)"
    print ""
    print $"  Push to Gitea \(if configured\):"
    print $"    git push gitea main"
    print $"    git push gitea ($tag_name)"
    print ""
    print $"  Publish to crates.io manually:"
    print $"    cargo publish"
    print ""
}
