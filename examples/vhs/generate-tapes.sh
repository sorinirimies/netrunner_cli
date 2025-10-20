#!/usr/bin/env bash

# VHS Tape Generator Script
# Generates all VHS terminal recordings in the examples/vhs directory
#
# Usage:
#   ./generate-all.sh           # Generate all tapes
#   ./generate-all.sh --clean   # Clean target directory first
#   ./generate-all.sh --help    # Show help

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/target"

# Function to print colored output
print_header() {
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_progress() {
    echo -e "${MAGENTA}▶${NC} $1"
}

# Function to show help
show_help() {
    cat << EOF
VHS Tape Generator Script

Usage:
  ./generate-all.sh [OPTIONS]

Options:
  --clean     Clean target directory before generating
  --help      Show this help message

Description:
  Generates all VHS terminal recordings from .tape files in the current directory.
  Output GIFs are saved to the target/ directory.

Examples:
  ./generate-all.sh           # Generate all tapes
  ./generate-all.sh --clean   # Clean and regenerate

Requirements:
  - VHS must be installed (https://github.com/charmbracelet/vhs)
  - NetRunner CLI must be built (cargo build --release)

EOF
}

# Function to check if VHS is installed
check_vhs() {
    if ! command -v vhs &> /dev/null; then
        print_error "VHS is not installed"
        echo ""
        echo "Install VHS:"
        echo "  macOS:   brew install vhs"
        echo "  Linux:   go install github.com/charmbracelet/vhs@latest"
        echo "  Other:   https://github.com/charmbracelet/vhs/releases"
        echo ""
        exit 1
    fi

    local vhs_version=$(vhs --version 2>&1 | head -n1)
    print_success "VHS is installed: ${vhs_version}"
}

# Function to clean target directory
clean_target() {
    if [ -d "${TARGET_DIR}" ]; then
        print_info "Cleaning target directory..."
        rm -rf "${TARGET_DIR}"/*.gif 2>/dev/null || true
        print_success "Target directory cleaned"
    fi
}

# Function to create target directory
ensure_target_dir() {
    if [ ! -d "${TARGET_DIR}" ]; then
        mkdir -p "${TARGET_DIR}"
        print_info "Created target directory"
    fi
}

# Function to get total tape count
count_tapes() {
    find "${SCRIPT_DIR}" -maxdepth 1 -name "*.tape" -type f | wc -l
}

# Function to format duration
format_duration() {
    local duration=$1
    if [ $duration -lt 60 ]; then
        echo "${duration}s"
    else
        local minutes=$((duration / 60))
        local seconds=$((duration % 60))
        echo "${minutes}m ${seconds}s"
    fi
}

# Function to get file size in human-readable format
get_file_size() {
    local file=$1
    if [ -f "$file" ]; then
        if command -v numfmt &> /dev/null; then
            numfmt --to=iec-i --suffix=B "$(stat -c%s "$file" 2>/dev/null || stat -f%z "$file" 2>/dev/null)"
        else
            ls -lh "$file" | awk '{print $5}'
        fi
    else
        echo "N/A"
    fi
}

# Function to generate a single tape
generate_tape() {
    local tape_file=$1
    local tape_name=$(basename "$tape_file" .tape)
    local output_file="${TARGET_DIR}/${tape_name}.gif"

    print_progress "Recording: ${tape_name}.tape"

    local start_time=$(date +%s)

    # Run VHS
    if vhs "$tape_file" 2>&1 | sed 's/^/  │ /'; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        local file_size=$(get_file_size "$output_file")

        print_success "Generated: ${tape_name}.gif (${file_size}) in $(format_duration $duration)"
        return 0
    else
        print_error "Failed to generate: ${tape_name}.tape"
        return 1
    fi
}

# Main function
main() {
    local clean_first=false

    # Parse arguments
    for arg in "$@"; do
        case $arg in
            --clean)
                clean_first=true
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                print_error "Unknown option: $arg"
                echo ""
                show_help
                exit 1
                ;;
        esac
    done

    # Change to script directory
    cd "${SCRIPT_DIR}"

    # Print header
    print_header "🎬 VHS Tape Generator"
    echo ""

    # Check prerequisites
    check_vhs
    echo ""

    # Clean if requested
    if [ "$clean_first" = true ]; then
        clean_target
        echo ""
    fi

    # Ensure target directory exists
    ensure_target_dir

    # Find all tape files
    tape_files=(*.tape)

    if [ ${#tape_files[@]} -eq 0 ] || [ ! -f "${tape_files[0]}" ]; then
        print_warning "No .tape files found in ${SCRIPT_DIR}"
        exit 0
    fi

    local total_tapes=${#tape_files[@]}
    print_info "Found ${total_tapes} tape file(s) to generate"
    echo ""

    # Generate each tape
    local success_count=0
    local fail_count=0
    local overall_start=$(date +%s)

    for i in "${!tape_files[@]}"; do
        local tape="${tape_files[$i]}"
        local current=$((i + 1))

        echo -e "${CYAN}[$current/$total_tapes]${NC}"

        if generate_tape "$tape"; then
            ((success_count++))
        else
            ((fail_count++))
        fi

        echo ""
    done

    # Print summary
    local overall_end=$(date +%s)
    local total_duration=$((overall_end - overall_start))

    print_header "📊 Generation Summary"
    echo ""
    print_info "Total tapes:     ${total_tapes}"
    print_success "Successful:      ${success_count}"

    if [ $fail_count -gt 0 ]; then
        print_error "Failed:          ${fail_count}"
    fi

    print_info "Total time:      $(format_duration $total_duration)"
    print_info "Output location: ${TARGET_DIR}"
    echo ""

    # List generated files
    if [ $success_count -gt 0 ]; then
        print_info "Generated files:"
        for gif in "${TARGET_DIR}"/*.gif; do
            if [ -f "$gif" ]; then
                local name=$(basename "$gif")
                local size=$(get_file_size "$gif")
                echo "  • ${name} (${size})"
            fi
        done
        echo ""
    fi

    # Exit with appropriate code
    if [ $fail_count -gt 0 ]; then
        print_warning "Some tapes failed to generate"
        exit 1
    else
        print_success "All tapes generated successfully! 🎉"
        exit 0
    fi
}

# Run main function
main "$@"
