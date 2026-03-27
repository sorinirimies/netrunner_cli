#!/usr/bin/env nu
# Migrate netrunner_cli to support dual hosting with GitHub and Gitea.
# Adds the Gitea remote, optionally updates the justfile with Gitea commands,
# and optionally pushes all branches and tags.
#
# Usage:   nu scripts/migrate_to_gitea.nu [project_dir] [gitea_url]
# Example: nu scripts/migrate_to_gitea.nu . git@gitea.example.com:username/netrunner_cli.git
#          nu scripts/migrate_to_gitea.nu (interactive — prompts for inputs)

def main [
    project_dir?: string,  # Path to repository root (defaults to current directory)
    gitea_url?: string,    # SSH or HTTPS Gitea URL (prompted if omitted)
] {
    let green  = (ansi green)
    let yellow = (ansi yellow)
    let cyan   = (ansi cyan)
    let red    = (ansi red)
    let blue   = (ansi blue)
    let reset  = (ansi reset)

    # ── Banner ────────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print $"($cyan)  netrunner_cli — Gitea migration($reset)"
    print $"($cyan)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print ""

    # ── Resolve project directory ─────────────────────────────────────────────
    let raw_dir = if ($project_dir | is-empty) {
        let answer = (input "  Enter project directory (or '.' for current): " | str trim)
        if ($answer | is-empty) { "." } else { $answer }
    } else {
        $project_dir
    }

    let resolved_dir = ($raw_dir | path expand)

    if not ($resolved_dir | path exists) {
        print $"($red)✘ Error: directory does not exist: ($resolved_dir)($reset)"
        exit 1
    }
    if not (($resolved_dir | path join ".git") | path exists) {
        print $"($red)✘ Error: not a git repository: ($resolved_dir)($reset)"
        exit 1
    }

    let project_name = ($resolved_dir | path basename)

    # ── Resolve Gitea URL ─────────────────────────────────────────────────────
    let resolved_url = if ($gitea_url | is-empty) {
        print $"  ($blue)ℹ($reset) Enter the Gitea repository URL (SSH is recommended)."
        print $"      Example: git@gitea.example.com:username/($project_name).git"
        print ""
        input "  Gitea URL: " | str trim
    } else {
        $gitea_url
    }

    if ($resolved_url | is-empty) {
        print $"($red)✘ Error: a Gitea URL is required.($reset)"
        exit 1
    }

    print ""
    print $"  ($blue)ℹ($reset) Project  : ($cyan)($project_name)($reset)"
    print $"  ($blue)ℹ($reset) Directory: ($resolved_dir)"
    print $"  ($blue)ℹ($reset) Gitea URL: ($resolved_url)"
    print ""

    # ── Change to project directory ───────────────────────────────────────────
    cd $resolved_dir

    # ─────────────────────────────────────────────────────────────────────────
    # Detect SSH vs HTTPS
    # ─────────────────────────────────────────────────────────────────────────
    let use_ssh = ($resolved_url | str starts-with "git@")

    let gitea_host = if $use_ssh {
        $resolved_url | str replace --regex '^git@([^:]+):.+$' '$1'
    } else {
        print $"  ($yellow)⚠($reset) HTTPS URL detected — SSH is strongly recommended to avoid credential prompts."
        $resolved_url | str replace --regex '^https?://([^/]+)/.+$' '$1'
    }

    # ─────────────────────────────────────────────────────────────────────────
    # SSH key check
    # ─────────────────────────────────────────────────────────────────────────
    if $use_ssh {
        print $"($cyan)[ssh]($reset) Checking SSH configuration …"

        let has_key = (
            ["~/.ssh/id_ed25519" "~/.ssh/id_rsa" "~/.ssh/id_ecdsa"]
            | any { |p| ($p | path expand | path exists) }
        )

        if not $has_key {
            print $"  ($yellow)⚠($reset) No SSH keys found."
            print ""
            print $"  ($blue)ℹ($reset) Generate a key with:"
            print "      ssh-keygen -t ed25519 -C \"your_email@example.com\""
            print ""
            let cont = (input "  Continue without SSH keys? [y/N] " | str trim | str downcase)
            if $cont != "y" and $cont != "yes" {
                print $"($red)✘ Error: SSH keys are required. Please set up SSH first.($reset)"
                exit 1
            }
        } else {
            print $"  ($green)✔($reset) SSH key found"

            print $"  ($blue)ℹ($reset) Testing SSH connection to ($gitea_host) …"
            let ssh_test = (
                do { run-external "ssh" "-o" "ConnectTimeout=5" "-o" "BatchMode=yes" "-T" $"git@($gitea_host)" }
                | complete
            )
            let combined = $"($ssh_test.stdout)($ssh_test.stderr)"
            if ($combined | str contains "successfully authenticated") or ($combined | str contains "Hi ") {
                print $"  ($green)✔($reset) SSH connection to ($gitea_host) successful"
            } else {
                print $"  ($yellow)⚠($reset) Could not verify SSH connection to ($gitea_host)."
                print ""
                print $"  ($blue)ℹ($reset) Make sure your SSH public key is added to Gitea:"
                print "      1. Copy  : cat ~/.ssh/id_ed25519.pub"
                print "      2. Paste : Gitea → Settings → SSH / GPG Keys → Add Key"
                print ""
            }
        }
        print ""
    }

    # ─────────────────────────────────────────────────────────────────────────
    # Add or update the gitea remote
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[remote]($reset) Configuring gitea remote …"

    let remote_list = (do { run-external "git" "remote" } | complete).stdout | str trim

    if ($remote_list | str contains "gitea") {
        print $"  ($yellow)⚠($reset) Remote 'gitea' already exists — updating URL."
        run-external "git" "remote" "set-url" "gitea" $resolved_url
        print $"  ($green)✔($reset) Gitea remote URL updated"
    } else {
        run-external "git" "remote" "add" "gitea" $resolved_url
        print $"  ($green)✔($reset) Gitea remote added"
    }

    # ─────────────────────────────────────────────────────────────────────────
    # Show configured remotes
    # ─────────────────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)[remotes]($reset) Currently configured remotes:"
    let remotes_out = (do { run-external "git" "remote" "-v" } | complete).stdout
    $remotes_out
    | lines
    | where { |l| ($l | str starts-with "origin") or ($l | str starts-with "gitea") }
    | each { |l| print $"    ($l)" }
    print ""

    # ─────────────────────────────────────────────────────────────────────────
    # Test repository connectivity
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[connectivity]($reset) Testing Gitea repository …"
    let ls_remote = (do { run-external "git" "ls-remote" "gitea" } | complete)
    if $ls_remote.exit_code == 0 {
        print $"  ($green)✔($reset) Successfully connected to Gitea repository!"
    } else {
        print $"  ($yellow)⚠($reset) Could not connect — the repository may not exist yet."
        print ""
        print $"  ($blue)ℹ($reset) To create it on Gitea:"
        print "      1. Log in to your Gitea instance"
        print "      2. Click '+' → New Repository"
        print $"      3. Repository name: ($project_name)"
        print "      4. Do NOT initialise with README"
        print "      5. Click 'Create Repository'"
        print $"      6. Then run:  git push gitea --all"
        print ""
    }

    # ─────────────────────────────────────────────────────────────────────────
    # Optionally update justfile with Gitea commands
    # ─────────────────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)[justfile]($reset) Checking justfile …"

    let justfile_path = ($resolved_dir | path join "justfile")

    if ($justfile_path | path exists) {
        let justfile_content = (open --raw $justfile_path)
        if ($justfile_content | str contains "push-gitea") {
            print $"  ($green)✔($reset) Gitea commands already present in justfile"
        } else {
            print $"  ($blue)ℹ($reset) Gitea commands not found in justfile."
            let add_just = (
                input "  Add Gitea commands to justfile? [y/N] "
                | str trim
                | str downcase
            )
            if $add_just == "y" or $add_just == "yes" {
                # Backup
                let backup_path = ($resolved_dir | path join "justfile.backup")
                $justfile_content | save --force $backup_path
                print $"  ($green)✔($reset) Backed up justfile to justfile.backup"

                # Append Gitea commands
                let gitea_commands = "
# ============================================================================
# Gitea Dual-Hosting Commands
# ============================================================================

# Push to GitHub (origin)
push:
    git push origin main

# Push to Gitea
push-gitea:
    git push gitea main

# Push to both GitHub and Gitea
push-all:
    git push origin main
    git push gitea main
    @echo \"✅ Pushed to both GitHub and Gitea!\"

# Push tags to GitHub
push-tags:
    git push origin --tags

# Push tags to both remotes
push-tags-all:
    git push origin --tags
    git push gitea --tags
    @echo \"✅ Tags pushed to both GitHub and Gitea!\"

# Release: push commit and tag to both remotes
push-release-all:
    git push origin main
    git push gitea main
    git push origin --tags
    git push gitea --tags
    @echo \"✅ Release pushed to both remotes!\"

# Force-sync Gitea with GitHub
sync-gitea:
    git push gitea main --force
    git push gitea --tags --force
    @echo \"✅ Gitea synced!\"

# Show configured remotes
remotes:
    @git remote -v
"
                ($justfile_content + $gitea_commands) | save --force $justfile_path
                print $"  ($green)✔($reset) Gitea commands appended to justfile"
                print $"  ($blue)ℹ($reset) Run ($yellow)just --list($reset) to see the new commands"
            } else {
                print $"  ($blue)ℹ($reset) Skipped — justfile unchanged"
            }
        }
    } else {
        print $"  ($yellow)⚠($reset) No justfile found — skipping justfile update."
        print $"  ($blue)ℹ($reset) Install just and run ($yellow)just --list($reset) to see available commands."
    }

    # ─────────────────────────────────────────────────────────────────────────
    # Optional push to Gitea
    # ─────────────────────────────────────────────────────────────────────────
    print ""
    let push_now = (
        input "Push all branches and tags to Gitea now? [y/N] "
        | str trim
        | str downcase
    )

    if $push_now == "y" or $push_now == "yes" {
        print ""
        print $"($cyan)[push]($reset) Pushing to Gitea …"

        let push_branches = (do { run-external "git" "push" "gitea" "--all" } | complete)
        if $push_branches.exit_code == 0 {
            print $"  ($green)✔($reset) All branches pushed to Gitea"
        } else {
            print $"  ($yellow)⚠($reset) Failed to push branches — the repository may not exist yet."
        }

        let push_tags = (do { run-external "git" "push" "gitea" "--tags" } | complete)
        if $push_tags.exit_code == 0 {
            print $"  ($green)✔($reset) All tags pushed to Gitea"
        } else {
            print $"  ($yellow)⚠($reset) Failed to push tags."
        }
    } else {
        print $"  ($blue)ℹ($reset) Skipped push. Run the commands below when ready."
    }

    # ─────────────────────────────────────────────────────────────────────────
    # Summary
    # ─────────────────────────────────────────────────────────────────────────
    print ""
    print $"($green)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print $"($green)  Migration complete! 🚀($reset)"
    print $"($green)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print ""
    print $"($cyan)Quick commands:($reset)"
    print "  Push to GitHub :          git push origin main"
    print "  Push to Gitea  :          git push gitea main"
    print "  Push to both   :          git push origin main && git push gitea main"
    print "  Push all tags  :          git push origin --tags && git push gitea --tags"
    print "  Sync Gitea     :          git push gitea main --force && git push gitea --tags --force"
    print "  View remotes   :          git remote -v"
    print ""
    if $use_ssh {
        print $"($green)  SSH configured — no passwords needed! 🔑($reset)"
    } else {
        print $"  ($blue)ℹ($reset) Consider switching to SSH to avoid credential prompts:"
        print $"      git remote set-url gitea git@($gitea_host):username/($project_name).git"
    }
    print ""
}
