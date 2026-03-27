#!/usr/bin/env nu
# Set up Gitea as a second remote for netrunner_cli (dual GitHub + Gitea hosting).
# Adds or updates the "gitea" remote, tests the connection, and optionally pushes
# all branches and tags.
#
# Usage:   nu scripts/setup_gitea.nu <gitea_url>
# Example: nu scripts/setup_gitea.nu git@gitea.example.com:username/netrunner_cli.git
#          nu scripts/setup_gitea.nu https://gitea.example.com/username/netrunner_cli.git

def main [
    gitea_url: string,  # SSH or HTTPS URL of the Gitea repository
] {
    let green  = (ansi green)
    let yellow = (ansi yellow)
    let cyan   = (ansi cyan)
    let red    = (ansi red)
    let blue   = (ansi blue)
    let reset  = (ansi reset)

    print ""
    print $"($cyan)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print $"($cyan)  netrunner_cli — Gitea dual-hosting setup($reset)"
    print $"($cyan)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print ""
    print $"  ($blue)ℹ($reset) Gitea URL : ($gitea_url)"
    print ""

    # ─────────────────────────────────────────────────────────────────────────
    # Preflight checks
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[preflight]($reset) Running checks …"

    # git installed?
    let git_check = (do { run-external "git" "--version" } | complete)
    if $git_check.exit_code != 0 {
        print $"($red)✘ Error: git is not installed. Please install git and try again.($reset)"
        exit 1
    }
    print $"  ($green)✔($reset) git is installed"

    # inside a git repository?
    let repo_check = (do { run-external "git" "rev-parse" "--git-dir" } | complete)
    if $repo_check.exit_code != 0 {
        print $"($red)✘ Error: Not a git repository. Run this script from the netrunner_cli directory.($reset)"
        exit 1
    }
    print $"  ($green)✔($reset) Inside a git repository"

    # ─────────────────────────────────────────────────────────────────────────
    # Detect SSH vs HTTPS and extract host
    # ─────────────────────────────────────────────────────────────────────────
    let use_ssh = ($gitea_url | str starts-with "git@")

    let gitea_host = if $use_ssh {
        # git@hostname:user/repo.git  →  hostname
        $gitea_url | str replace --regex '^git@([^:]+):.+$' '$1'
    } else {
        print $"  ($yellow)⚠($reset) HTTPS URL detected — SSH is strongly recommended to avoid credential prompts."
        # https://hostname/user/repo.git  →  hostname
        $gitea_url | str replace --regex '^https?://([^/]+)/.+$' '$1'
    }

    # ─────────────────────────────────────────────────────────────────────────
    # SSH key check (only when using SSH)
    # ─────────────────────────────────────────────────────────────────────────
    if $use_ssh {
        print ""
        print $"($cyan)[ssh]($reset) Checking SSH configuration …"

        let has_key = (
            ["~/.ssh/id_ed25519" "~/.ssh/id_rsa" "~/.ssh/id_ecdsa"]
            | any { |p| ($p | path expand | path exists) }
        )

        if not $has_key {
            print $"  ($yellow)⚠($reset) No SSH keys found."
            print ""
            print $"  ($blue)ℹ($reset) Generate one with:"
            print "      ssh-keygen -t ed25519 -C \"your_email@example.com\""
            print ""
            let answer = (input "  Continue without SSH keys? [y/N] " | str trim | str downcase)
            if $answer != "y" and $answer != "yes" {
                print $"($red)✘ Error: SSH keys required. Please set up SSH first.($reset)"
                exit 1
            }
        } else {
            print $"  ($green)✔($reset) SSH key found"

            # Test SSH connection to Gitea host
            print $"  ($blue)ℹ($reset) Testing SSH connection to ($gitea_host) …"
            let ssh_test = (
                do { run-external "ssh" "-o" "ConnectTimeout=5" "-o" "BatchMode=yes" "-T" $"git@($gitea_host)" }
                | complete
            )
            # Gitea echoes "Hi <user>! You've successfully authenticated"
            # Exit code is often non-zero even on success, so inspect output text
            let combined = $"($ssh_test.stdout)($ssh_test.stderr)"
            if ($combined | str contains "successfully authenticated") or ($combined | str contains "Hi ") {
                print $"  ($green)✔($reset) SSH connection successful"
            } else {
                print $"  ($yellow)⚠($reset) Could not verify SSH connection."
                print ""
                print $"  ($blue)ℹ($reset) Ensure your public key is added to Gitea:"
                print "      1. Copy : cat ~/.ssh/id_ed25519.pub"
                print "      2. Paste : Gitea → Settings → SSH / GPG Keys → Add Key"
                print ""
            }
        }
    }

    # ─────────────────────────────────────────────────────────────────────────
    # Add or update the gitea remote
    # ─────────────────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)[remote]($reset) Configuring gitea remote …"

    let remote_list = (do { run-external "git" "remote" } | complete).stdout | str trim

    if ($remote_list | str contains "gitea") {
        print $"  ($yellow)⚠($reset) Remote 'gitea' already exists — updating URL."
        run-external "git" "remote" "set-url" "gitea" $gitea_url
        print $"  ($green)✔($reset) Gitea remote URL updated"
    } else {
        run-external "git" "remote" "add" "gitea" $gitea_url
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
    # Test Gitea repository connectivity
    # ─────────────────────────────────────────────────────────────────────────
    print $"($cyan)[connectivity]($reset) Testing Gitea repository …"
    let ls_remote = (
        do { run-external "git" "ls-remote" "gitea" }
        | complete
    )
    if $ls_remote.exit_code == 0 {
        print $"  ($green)✔($reset) Successfully connected to Gitea repository!"
    } else {
        print $"  ($yellow)⚠($reset) Could not connect to Gitea repository."
        print ""
        print $"  ($blue)ℹ($reset) This is normal if the repository does not exist yet."
        print "      To create it on Gitea:"
        print "        1. Log in to your Gitea instance"
        print "        2. Click '+' → New Repository"
        print "        3. Repository name: netrunner_cli"
        print "        4. Do NOT initialize with README"
        print "        5. Click 'Create Repository'"
        print $"        6. Then run:  git push gitea --all"
        print ""
    }

    # ─────────────────────────────────────────────────────────────────────────
    # Optional push
    # ─────────────────────────────────────────────────────────────────────────
    print ""
    let push_answer = (
        input "Push all branches and tags to Gitea now? [y/N] "
        | str trim
        | str downcase
    )

    if $push_answer == "y" or $push_answer == "yes" {
        print ""
        print $"($cyan)[push]($reset) Pushing to Gitea …"

        # Push all branches
        let push_branches = (do { run-external "git" "push" "gitea" "--all" } | complete)
        if $push_branches.exit_code == 0 {
            print $"  ($green)✔($reset) All branches pushed to Gitea"
        } else {
            print $"  ($yellow)⚠($reset) Failed to push branches — the repository may not exist yet."
        }

        # Push all tags
        let push_tags = (do { run-external "git" "push" "gitea" "--tags" } | complete)
        if $push_tags.exit_code == 0 {
            print $"  ($green)✔($reset) All tags pushed to Gitea"
        } else {
            print $"  ($yellow)⚠($reset) Failed to push tags."
        }
    } else {
        print $"  ($blue)ℹ($reset) Skipped push. Push manually when ready."
    }

    # ─────────────────────────────────────────────────────────────────────────
    # Summary
    # ─────────────────────────────────────────────────────────────────────────
    print ""
    print $"($green)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print $"($green)  Gitea setup complete! 🎉($reset)"
    print $"($green)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print ""
    print $"($cyan)Quick commands:($reset)"
    print "  Push to GitHub :          git push origin main"
    print "  Push to Gitea  :          git push gitea main"
    print "  Push to both   :          git push origin main && git push gitea main"
    print "  Push tags (GitHub) :      git push origin --tags"
    print "  Push tags (both)   :      git push origin --tags && git push gitea --tags"
    print "  Sync Gitea (force) :      git push gitea main --force && git push gitea --tags --force"
    print "  View remotes       :      git remote -v"
    print ""
    if $use_ssh {
        print $"($green)  SSH setup — no passwords needed! 🔑($reset)"
    } else {
        print $"  ($blue)ℹ($reset) Switch to SSH to avoid credential prompts:"
        print $"      git remote set-url gitea git@($gitea_host):username/netrunner_cli.git"
    }
    print ""
}
