#!/usr/bin/env nu
# Update README.md version badge and regenerate CHANGELOG.md for a release tag.
# Called by the update-readme CI workflow after a version tag is pushed.
# The workflow handles git commit/push; this script only modifies the files.
#
# Usage:   nu scripts/update_docs.nu <tag>
# Example: nu scripts/update_docs.nu v0.7.0

def main [tag: string] {
    let green  = (ansi green)
    let yellow = (ansi yellow)
    let cyan   = (ansi cyan)
    let red    = (ansi red)
    let reset  = (ansi reset)

    # ── Derive bare version from tag ──────────────────────────────────────────
    let version = ($tag | str replace --regex '^v' '')

    print $"($cyan)update_docs($reset) — ($tag) → ($green)($version)($reset)"
    print ""

    # ─────────────────────────────────────────────────────────────────────────
    # Step 1 — Update README.md version badge
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[1/2]($reset) Updating README.md version badge …"

    if not ("README.md" | path exists) {
        print $"  ($red)✘ README.md not found in current directory.($reset)"
        exit 1
    }

    let readme = (
        open --raw README.md
        | str replace --all --regex 'version-[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?-blue' $"version-($version)-blue"
    )
    $readme | save --force README.md
    print $"  ($green)✔ README.md badge updated to ($version)($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Step 2 — Regenerate CHANGELOG.md with git-cliff
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[2/2]($reset) Regenerating CHANGELOG.md …"

    let cliff = (
        do { run-external "git-cliff" "--tag" $tag "-o" "CHANGELOG.md" }
        | complete
    )

    if $cliff.exit_code != 0 {
        print $"  ($yellow)⚠ git-cliff exited ($cliff.exit_code) — CHANGELOG.md may be incomplete.($reset)"
        print $"  Install git-cliff with: ($yellow)cargo install git-cliff($reset)"
    } else {
        print $"  ($green)✔ CHANGELOG.md regenerated($reset)"
    }

    # ─────────────────────────────────────────────────────────────────────────
    # Summary
    # ─────────────────────────────────────────────────────────────────────────
    print ""
    print $"($green)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print $"($green)  Documentation updated for ($tag) ✅($reset)"
    print $"($green)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print ""
    print $"($cyan)Files modified:($reset)"
    print $"  ($green)✔($reset) README.md    — version badge set to ($version)"
    print $"  ($green)✔($reset) CHANGELOG.md — regenerated for ($tag)"
    print ""
    print $"($cyan)The CI workflow will commit and push these changes.($reset)"
    print ""
}
