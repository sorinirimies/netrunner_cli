#!/usr/bin/env nu
# Prepare release artifacts for netrunner_cli.
# Called by the CI release workflow when a version tag is pushed.
#
# Usage:  nu scripts/release_prepare.nu <tag>
# Example: nu scripts/release_prepare.nu v0.7.0

# Build the Markdown release notes from component parts.
# Returns a single string with lines joined by newlines.
def build_release_notes [
    version: string,       # Bare version, e.g. "0.7.0"
    last_tag: string,      # Previous git tag, or "" for initial release
    cliff_changes: string, # Body text produced by git-cliff
] {
    let what_changed = if ($last_tag | is-empty) {
        "### 🎉 Initial Release"
    } else {
        $"### 📝 Changes since ($last_tag):"
    }

    let lines = [
        $"# NetRunner CLI ($version)"
        ""
        "## 🚀 What's New"
        ""
        $what_changed
        ""
        $cliff_changes
        ""
        "## 📦 Installation"
        ""
        "```bash"
        "cargo install netrunner_cli"
        "```"
        ""
        "## 🚀 Quick Start"
        ""
        "```bash"
        "# Run interactive menu"
        "netrunner_cli"
        ""
        "# Run speed test directly"
        "netrunner_cli -m speed"
        "```"
    ]

    $lines | str join "\n"
}

def main [tag: string] {
    let green  = (ansi green)
    let yellow = (ansi yellow)
    let cyan   = (ansi cyan)
    let red    = (ansi red)
    let reset  = (ansi reset)

    # Strip leading 'v' to obtain the bare version string
    let version = ($tag | str replace --regex '^v' '')

    print $"($cyan)release_prepare($reset) — ($tag) → ($green)($version)($reset)"
    print ""

    # ─────────────────────────────────────────────────────────────────────────
    # Step 1 — Update Cargo.toml version
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[1/5]($reset) Updating Cargo.toml to ($version) …"
    let cargo_toml = (
        open --raw Cargo.toml
        | str replace --regex '(?m)^version = "[^"]*"' $"version = \"($version)\""
    )
    $cargo_toml | save --force Cargo.toml
    print $"  ($green)✔ Cargo.toml updated($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Step 2 — Regenerate full CHANGELOG.md
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[2/5]($reset) Regenerating CHANGELOG.md …"
    let changelog = (
        do { run-external "git-cliff" "--config" "cliff.toml" "--output" "CHANGELOG.md" }
        | complete
    )
    if $changelog.exit_code != 0 {
        print $"  ($yellow)⚠ git-cliff exited ($changelog.exit_code) — CHANGELOG.md may be incomplete.($reset)"
    } else {
        print $"  ($green)✔ CHANGELOG.md generated($reset)"
    }

    # ─────────────────────────────────────────────────────────────────────────
    # Step 3 — Generate per-release diff into CLIFF_CHANGES.md
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[3/5]($reset) Generating per-release changes …"

    let last_tag_result = (
        do { run-external "git" "describe" "--tags" "--abbrev=0" "HEAD^" }
        | complete
    )
    let last_tag = if $last_tag_result.exit_code == 0 {
        $last_tag_result.stdout | str trim
    } else {
        ""
    }

    if ($last_tag | is-empty) {
        print $"  ($yellow)No previous tag found — treating as initial release.($reset)"
        let cliff_init = (
            do { run-external "git-cliff" "--config" "cliff.toml" "--tag" $tag "--strip" "header" "--output" "CLIFF_CHANGES.md" }
            | complete
        )
        if $cliff_init.exit_code != 0 {
            print $"  ($yellow)⚠ git-cliff \(initial\) exited ($cliff_init.exit_code) — using empty diff.($reset)"
            "" | save --force CLIFF_CHANGES.md
        }
    } else {
        print $"  Previous tag: ($yellow)($last_tag)($reset)"
        let range = $"($last_tag)..($tag)"
        let cliff_diff = (
            do { run-external "git-cliff" "--config" "cliff.toml" $range "--strip" "header" "--output" "CLIFF_CHANGES.md" }
            | complete
        )
        if $cliff_diff.exit_code != 0 {
            print $"  ($yellow)⚠ git-cliff \(diff\) exited ($cliff_diff.exit_code) — using empty diff.($reset)"
            "" | save --force CLIFF_CHANGES.md
        }
    }
    print $"  ($green)✔ CLIFF_CHANGES.md generated($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Step 4 — Build RELEASE_NOTES.md
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[4/5]($reset) Building RELEASE_NOTES.md …"

    let cliff_changes = if ("CLIFF_CHANGES.md" | path exists) {
        open --raw CLIFF_CHANGES.md | str trim
    } else {
        ""
    }

    let notes = (build_release_notes $version $last_tag $cliff_changes)
    $notes | save --force RELEASE_NOTES.md
    print $"  ($green)✔ RELEASE_NOTES.md written($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Step 5 — Clean up temporary CLIFF_CHANGES.md
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[5/5]($reset) Cleaning up temporary files …"
    if ("CLIFF_CHANGES.md" | path exists) {
        rm CLIFF_CHANGES.md
    }
    print $"  ($green)✔ CLIFF_CHANGES.md removed($reset)"

    # ─────────────────────────────────────────────────────────────────────────
    # Summary
    # ─────────────────────────────────────────────────────────────────────────
    print ""
    print $"($green)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print $"($green)  Release ($tag) prepared successfully! 🚀($reset)"
    print $"($green)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print ""
    print $"($cyan)Artifacts written:($reset)"
    print "  • Cargo.toml      — version updated"
    print "  • CHANGELOG.md    — full project changelog"
    print "  • RELEASE_NOTES.md — per-release notes (use as GitHub Release body)"
    print ""
}
