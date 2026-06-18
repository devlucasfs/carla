#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Log file
LOG_FILE="${HOME}/.carla_installer.log"
INSTALL_TIMESTAMP=$(date '+%Y%m%d_%H%M%S')

# Default values
ARCH="${1:-auto}"
OS="${2:-auto}"
INSTALL_BASE="${HOME}/.carla"
INSTALL_BIN="${INSTALL_BASE}/bin"
INSTALL_EXTENSORS="${INSTALL_BASE}/extensors"

# ----------------------------------------------------------------------
#  LOGGING FUNCTIONS
# ----------------------------------------------------------------------
log() {
    local level="$1"
    local message="$2"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')

    echo -e "${level}${message}${NC}" >&2
    echo "[$timestamp] $level$message" >> "$LOG_FILE"
}

info() { log "${BLUE}" "INFO: $1"; }
success() { log "${GREEN}" "SUCCESS: $1"; }
warning() { log "${YELLOW}" "WARNING: $1"; }
error() { log "${RED}" "ERROR: $1"; }

# ----------------------------------------------------------------------
#  ERROR HANDLING AND CLEANUP
# ----------------------------------------------------------------------
cleanup() {
    if [ $? -ne 0 ]; then
        error "Installation failed! Check log: $LOG_FILE"
    fi

    # Cleanup temporary directory
    if [ -n "${TEMP_DIR:-}" ] && [ -d "$TEMP_DIR" ]; then
        info "Cleaning up temporary files..."
        rm -rf "$TEMP_DIR"
    fi
}

trap cleanup EXIT

# ----------------------------------------------------------------------
#  ARCHITECTURE AND OS DETECTION (with FreeBSD support)
# ----------------------------------------------------------------------
detect_arch() {
    local arch=$(uname -m 2>/dev/null || echo "unknown")
    case "$arch" in
        x86_64|amd64)
            echo "x86_64"
            ;;
        aarch64|arm64)
            echo "aarch64"
            ;;
        armv7l|armv7|armhf)
            echo "armv7"
            ;;
        i386|i686)
            echo "i386"
            ;;
        *)
            echo "$arch"
            ;;
    esac
}

detect_os() {
    local os=$(uname -s 2>/dev/null | tr '[:upper:]' '[:lower:]')
    case "$os" in
        linux)
            echo "linux"
            ;;
        darwin)
            echo "macos"
            ;;
        freebsd)
            echo "freebsd"
            ;;
        openbsd)
            echo "openbsd"
            ;;
        netbsd)
            echo "netbsd"
            ;;
        dragonfly)
            echo "dragonfly"
            ;;
        *)
            echo "$os"
            ;;
    esac
}

# ----------------------------------------------------------------------
#  PACKAGE MANAGER DETECTION (with FreeBSD support)
# ----------------------------------------------------------------------
detect_package_manager() {
    if command -v pkg >/dev/null 2>&1 && [ "$(detect_os)" = "freebsd" ]; then
        echo "pkg"
    elif command -v apt >/dev/null 2>&1; then
        echo "apt"
    elif command -v dnf >/dev/null 2>&1; then
        echo "dnf"
    elif command -v yum >/dev/null 2>&1; then
        echo "yum"
    elif command -v pacman >/dev/null 2>&1; then
        echo "pacman"
    elif command -v brew >/dev/null 2>&1; then
        echo "brew"
    else
        echo "unknown"
    fi
}

# ----------------------------------------------------------------------
#  DOWNLOAD FUNCTIONS
# ----------------------------------------------------------------------
download_so_files() {
    local arch="$1"
    local os="$2"
    local temp_dir="$3"

    info "Downloading library files for ${arch}-${os}..."

    # Construct base URL
    local base_url="https://carla-cdn.vercel.app"
    local files=("eva" "runa")
    local downloaded=0

    for file in "${files[@]}"; do
        # Try different naming conventions
        local names=(
            "lib${file}-${os}-${arch}.so"
            "lib${file}-${arch}-${os}.so"
            "lib${file}.${os}.${arch}.so"
            "lib${file}_${os}_${arch}.so"
            "${file}-${os}-${arch}.so"
            "${file}-${arch}-${os}.so"
        )

        local downloaded_file=""

        for name in "${names[@]}"; do
            local url="${base_url}/${name}"
            local output="${temp_dir}/${name}"

            info "Trying: ${name}"

            if curl -L -f -s -o "$output" "$url" 2>>"$LOG_FILE"; then
                if [ -f "$output" ] && [ -s "$output" ]; then
                    chmod +x "$output"
                    success "Downloaded: ${name}"
                    downloaded_file="$output"
                    ((downloaded++))
                    break
                fi
            fi
        done

        if [ -z "$downloaded_file" ]; then
            warning "Failed to download ${file} for ${os}-${arch}"
        fi
    done

    if [ $downloaded -eq 0 ]; then
        error "No library files were successfully downloaded"
        return 1
    fi

    success "Downloaded ${downloaded} library files"

    # Copy library files to bin directory (alongside binaries)
    mkdir -p "$INSTALL_BIN"
    cp "${temp_dir}"/*.so "$INSTALL_BIN/" 2>/dev/null || true

    # Rename to standard names with lib prefix if needed
    pushd "$INSTALL_BIN" >/dev/null
    for file in *.so; do
        if [[ "$file" == *"eva"* ]] && [[ "$file" != "libeva.so" ]]; then
            if [[ "$file" == "eva"* ]]; then
                mv "$file" "libeva.so" 2>/dev/null || true
            else
                # If it already has a different name but contains eva
                mv "$file" "libeva.so" 2>/dev/null || true
            fi
        elif [[ "$file" == *"runa"* ]] && [[ "$file" != "libruna.so" ]]; then
            if [[ "$file" == "runa"* ]]; then
                mv "$file" "libruna.so" 2>/dev/null || true
            else
                mv "$file" "libruna.so" 2>/dev/null || true
            fi
        fi
    done
    popd >/dev/null

    info "Library files installed in: ${INSTALL_BIN} (same directory as binaries)"

    # Set library path
    export LD_LIBRARY_PATH="${INSTALL_BIN}:${LD_LIBRARY_PATH:-}"
    export DYLD_LIBRARY_PATH="${INSTALL_BIN}:${DYLD_LIBRARY_PATH:-}" # For macOS

    return 0
}

# ----------------------------------------------------------------------
#  VALIDATION FUNCTIONS
# ----------------------------------------------------------------------
check_commands() {
    local os_type=$(detect_os)
    local commands=("git" "gcc" "g++" "make" "cmake" "curl")

    # Different commands for different OS
    case "$os_type" in
        freebsd)
            commands+=("pkg")
            ;;
        linux)
            commands+=("pkg-config")
            ;;
        darwin|macos)
            commands+=("pkg-config")
            ;;
    esac

    local missing=()

    for cmd in "${commands[@]}"; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            missing+=("$cmd")
        fi
    done

    if [ ${#missing[@]} -gt 0 ]; then
        error "Missing required commands: ${missing[*]}"
        info "Please install them using your package manager:"

        local pkg_manager=$(detect_package_manager)
        case "$pkg_manager" in
            pkg)
                echo "  FreeBSD: sudo pkg install ${missing[*]}"
                ;;
            apt)
                echo "  Ubuntu/Debian: sudo apt install ${missing[*]}"
                ;;
            dnf|yum)
                echo "  Fedora/RHEL: sudo ${pkg_manager} install ${missing[*]}"
                ;;
            pacman)
                echo "  Arch: sudo pacman -S ${missing[*]}"
                ;;
            brew)
                echo "  macOS: brew install ${missing[*]}"
                ;;
            *)
                echo "  Please install missing packages manually"
                ;;
        esac
        return 1
    fi

    success "All required commands are available"
}

check_disk_space() {
    local required_mb=500
    local available_mb

    if [[ "$OSTYPE" == "freebsd"* ]]; then
        available_mb=$(df -m "$HOME" | awk 'NR==2 {print $4}')
    else
        available_mb=$(df "$HOME" | awk 'NR==2 {print $4}')
        available_mb=$((available_mb / 1024))
    fi

    if [ "$available_mb" -lt "$required_mb" ]; then
        error "Insufficient disk space. Required: ${required_mb}MB, Available: ${available_mb}MB"
        return 1
    fi

    success "Disk space check passed: ${available_mb}MB available"
}

# ----------------------------------------------------------------------
#  SHELL DETECTION AND PATH MANAGEMENT
# ----------------------------------------------------------------------
detect_shell() {
    local shell_path="${SHELL:-/bin/sh}"
    echo $(basename "$shell_path")
}

get_shell_rc() {
    local shell="$1"
    case "$shell" in
        *bash)
            echo "$HOME/.bashrc"
            ;;
        *zsh)
            if [ -n "${ZDOTDIR:-}" ]; then
                echo "${ZDOTDIR}/.zshrc"
            else
                echo "$HOME/.zshrc"
            fi
            ;;
        *fish)
            echo "$HOME/.config/fish/config.fish"
            ;;
        *)
            echo "$HOME/.profile"
            ;;
    esac
}

add_to_path() {
    local dir="$1"
    local current_shell=$(detect_shell)
    local shell_rc=$(get_shell_rc "$current_shell")

    info "Adding $dir to PATH"

    if [ ! -d "$dir" ]; then
        error "Directory does not exist: $dir"
        return 1
    fi

    # Add to PATH for current session
    export PATH="${dir}:$PATH"

    # Add to shell RC file if not already there
    if [ -f "$shell_rc" ]; then
        if ! grep -q "export PATH=.*${dir}" "$shell_rc" 2>/dev/null; then
            {
                echo ""
                echo "# Added by Carla installer - ${INSTALL_TIMESTAMP}"
                echo "export PATH=\"${dir}:\$PATH\""
                echo "export LD_LIBRARY_PATH=\"${dir}:\$LD_LIBRARY_PATH\""
                echo "export DYLD_LIBRARY_PATH=\"${dir}:\$DYLD_LIBRARY_PATH\""
            } >> "$shell_rc"
            success "Added to ${shell_rc}"
        fi
    elif [ "$current_shell" = "fish" ]; then
        # Fish shell uses different syntax
        mkdir -p "$(dirname "$shell_rc")"
        {
            echo ""
            echo "# Added by Carla installer - ${INSTALL_TIMESTAMP}"
            echo "set -gx PATH \"${dir}\" \$PATH"
            echo "set -gx LD_LIBRARY_PATH \"${dir}\" \$LD_LIBRARY_PATH"
        } >> "$shell_rc"
        success "Added to ${shell_rc}"
    else
        # Create RC file if it doesn't exist
        mkdir -p "$(dirname "$shell_rc")"
        {
            echo "# Created by Carla installer - ${INSTALL_TIMESTAMP}"
            echo "export PATH=\"${dir}:\$PATH\""
            echo "export LD_LIBRARY_PATH=\"${dir}:\$LD_LIBRARY_PATH\""
            echo "export DYLD_LIBRARY_PATH=\"${dir}:\$DYLD_LIBRARY_PATH\""
        } > "$shell_rc"
        success "Created ${shell_rc}"
    fi

    success "Added ${dir} to PATH"
}

# ----------------------------------------------------------------------
#  BUILD FUNCTIONS
# ----------------------------------------------------------------------
build_repository() {
    local repo_url="$1"
    local repo_name="$2"
    local arch="$3"
    local os="$4"
    local temp_dir="$5"

    info "Building ${repo_name} for ${os}-${arch}..."

    local repo_dir="${temp_dir}/${repo_name}"

    # Try different build script names
    local scripts=(
        "scripts/${arch}-${os}-build.sh"
        "scripts/${os}-${arch}-build.sh"
    )

    # Clone repository
    if ! git clone --depth 1 "$repo_url" "$repo_dir" 2>>"$LOG_FILE"; then
        error "Failed to clone ${repo_name} repository"
        return 1
    fi

    pushd "$repo_dir" >/dev/null

    local found_script=""
    for script in "${scripts[@]}"; do
        if [ -f "$script" ]; then
            found_script="$script"
            info "Using build script: $script"
            break
        fi
    done

    if [ -z "$found_script" ]; then
        error "No build script found for ${os}-${arch}"
        info "Available scripts:"
        find . -name "*.sh" -type f 2>/dev/null | head -10
        popd >/dev/null
        return 1
    fi

    chmod +x "$found_script"

    # Set library path for build (bin directory contains the .so files)
    export LD_LIBRARY_PATH="${INSTALL_BIN}:${LD_LIBRARY_PATH:-}"
    export DYLD_LIBRARY_PATH="${INSTALL_BIN}:${DYLD_LIBRARY_PATH:-}"

    if ! ./"$found_script" >> "$LOG_FILE" 2>&1; then
        error "${repo_name} build failed - check log: $LOG_FILE"
        popd >/dev/null
        return 1
    fi

    # Find binary and copy to installation directory
    mkdir -p "$INSTALL_BIN"

    local binary_name="${repo_name}"
    local binary_path=$(find . -type f -executable -name "${binary_name}" 2>/dev/null | head -1)

    if [ -n "$binary_path" ] && [ -f "$binary_path" ]; then
        cp "$binary_path" "${INSTALL_BIN}/"
        success "Installed ${binary_name} to ${INSTALL_BIN}"
    else
        # Try to find any executable that might be the binary
        local potential_bin=$(find . -type f -executable -not -name "*.sh" -not -name "*.py" 2>/dev/null | head -1)
        if [ -n "$potential_bin" ]; then
            cp "$potential_bin" "${INSTALL_BIN}/${repo_name}"
            success "Installed ${repo_name} (from ${potential_bin}) to ${INSTALL_BIN}"
        else
            warning "Could not find ${binary_name} binary"
            # List potential binaries for debugging
            find . -type f -executable 2>/dev/null | head -10
        fi
    fi

    popd >/dev/null

    return 0
}

# ----------------------------------------------------------------------
#  INSTALLATION FUNCTIONS
# ----------------------------------------------------------------------
install_morgana() {
    local arch="$1"
    local os="$2"
    local temp_dir="$3"

    info "Installing Morgana for ${os}-${arch}..."

    if ! build_repository "https://github.com/devlucasfs/morgana.git" "morgana" "$arch" "$os" "$temp_dir"; then
        error "Morgana installation failed"
        return 1
    fi

    success "Morgana installed successfully"
    return 0
}

install_carla() {
    local arch="$1"
    local os="$2"
    local temp_dir="$3"

    info "Installing Carla for ${os}-${arch}..."

    if ! build_repository "https://github.com/devlucasfs/carla.git" "carla" "$arch" "$os" "$temp_dir"; then
        error "Carla installation failed"
        return 1
    fi

    success "Carla installed successfully"
    return 0
}

# ----------------------------------------------------------------------
#  VERIFICATION FUNCTIONS
# ----------------------------------------------------------------------
verify_installation() {
    info "Verifying installation..."

    local verified=true

    # Check directories
    if [ -d "$INSTALL_BIN" ]; then
        success "✓ Binaries directory: $INSTALL_BIN"
        ls -la "$INSTALL_BIN" | grep -v "^total" | while read -r line; do
            info "  - $(echo "$line" | awk '{print $NF}')"
        done
    else
        error "✗ Binaries directory not found"
        verified=false
    fi

    # Check extensors directory exists and is empty (or can be empty)
    if [ -d "$INSTALL_EXTENSORS" ]; then
        success "✓ Extensors directory: $INSTALL_EXTENSORS"
        local file_count=$(ls -1 "$INSTALL_EXTENSORS" 2>/dev/null | wc -l)
        if [ "$file_count" -eq 0 ]; then
            info "  Extensors directory is empty (ready for future extensions)"
        else
            info "  Extensors directory contains $file_count items"
        fi
    else
        warning "✗ Extensors directory not found (creating...)"
        mkdir -p "$INSTALL_EXTENSORS"
        success "Created extensors directory"
    fi

    # Check if commands are available
    if command -v carla >/dev/null 2>&1; then
        success "✓ Carla is available in PATH"
        local carla_path=$(which carla)
        info "  Location: $carla_path"
    else
        warning "✗ Carla not found in PATH"
        verified=false
    fi

    if command -v morgana >/dev/null 2>&1; then
        success "✓ Morgana is available in PATH"
        local morgana_path=$(which morgana)
        info "  Location: $morgana_path"
    else
        warning "✗ Morgana not found in PATH"
        verified=false
    fi

    # Check library files in bin directory (with lib prefix)
    if [ -f "${INSTALL_BIN}/libeva.so" ]; then
        success "✓ libeva.so library found in bin/"
    else
        warning "✗ libeva.so library missing from bin/"
        verified=false
    fi

    if [ -f "${INSTALL_BIN}/libruna.so" ]; then
        success "✓ libruna.so library found in bin/"
    else
        warning "✗ libruna.so library missing from bin/"
        verified=false
    fi

    if [ "$verified" = true ]; then
        success "All components verified successfully"
    else
        error "Some components failed verification"
    fi

    return 0
}

# ----------------------------------------------------------------------
#  USER PROMPTS
# ----------------------------------------------------------------------
prompt_yn() {
    local msg="$1"
    local default="${2:-Y}"
    local choice
    local prompt

    if [ "$default" = "Y" ]; then
        prompt="[Y/n]"
    else
        prompt="[y/N]"
    fi

    while true; do
        echo -n "$msg $prompt: "
        read -r choice
        choice="${choice:-$default}"

        case "${choice,,}" in
            y|yes) return 0 ;;
            n|no)  return 1 ;;
            *) echo "Please answer 'y' or 'n'" ;;
        esac
    done
}

# ----------------------------------------------------------------------
#  MAIN INSTALLATION LOGIC
# ----------------------------------------------------------------------
main() {
    # Auto-detect if not specified
    if [ "$ARCH" = "auto" ]; then
        ARCH=$(detect_arch)
        info "Auto-detected architecture: $ARCH"
    fi

    if [ "$OS" = "auto" ]; then
        OS=$(detect_os)
        info "Auto-detected OS: $OS"
    fi

    echo -e "${BLUE}"
    cat << "EOF"
    ___          _                     _   __  __
   / __|__ _ _ _| |__ _   __ _ _ _  __| | |  \/  |___ _ _ __ _ __ _ _ _  __ _
  | (__/ _` | '_| / _` | / _` | ' \/ _` | | |\/| / _ \ '_/ _` / _` | ' \/ _` |
   \___\__,_|_| |_\__,_| \__,_|_||_\__,_| |_|  |_\___/_| \__, \__,_|_||_\__,_|
EOF
    echo -e "${NC}"

    info "CARLA/Morgana Multi-Platform Installer"
    info "Target: ${OS}-${ARCH}"
    info "Install directory: ${INSTALL_BASE}"
    info "Log file: $LOG_FILE"

    # Show OS-specific information
    case "$OS" in
        freebsd)
            info "Running on FreeBSD - using compatibility mode"
            ;;
        linux)
            info "Running on Linux"
            ;;
        macos|darwin)
            info "Running on macOS"
            ;;
    esac

    # System checks
    info "Performing system checks..."
    check_commands || exit 1
    check_disk_space || exit 1

    # Remove old installation if exists
    if [ -d "$INSTALL_BASE" ]; then
        warning "Existing installation found at $INSTALL_BASE"
        if prompt_yn "Remove and reinstall?" "N"; then
            rm -rf "$INSTALL_BASE"
            info "Removed old installation"
        else
            info "Installation cancelled by user"
            exit 0
        fi
    fi

    # Create installation directories
    mkdir -p "$INSTALL_BIN"
    mkdir -p "$INSTALL_EXTENSORS"  # Empty directory for future extensions

    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    info "Created temporary directory: $TEMP_DIR"

    # Download library files (.so) to bin directory
    if ! download_so_files "$ARCH" "$OS" "$TEMP_DIR"; then
        warning "Failed to download some library files, but continuing with build..."
    fi

    # Install Morgana (dependency for Carla)
    info "Installing Morgana..."
    if ! install_morgana "$ARCH" "$OS" "$TEMP_DIR"; then
        error "Morgana installation failed - cannot continue"
        exit 1
    fi

    # Install Carla
    info "Installing Carla..."
    if ! install_carla "$ARCH" "$OS" "$TEMP_DIR"; then
        error "Carla installation failed"
        exit 1
    fi

    # Add to PATH
    add_to_path "$INSTALL_BIN"

    # Verification
    verify_installation

    # Success message
    echo
    success "🎉 Installation completed successfully!"
    echo
    info "Next steps:"
    echo "  1. Run: source $(get_shell_rc "$(detect_shell)")"
    echo "  2. Or simply restart your terminal"
    echo "  3. Test with: carla version && morgana version"
    echo
    info "Log file: $LOG_FILE"
}

# ----------------------------------------------------------------------
#  SCRIPT START
# ----------------------------------------------------------------------
show_usage() {
    cat << EOF
Usage: $0 [ARCHITECTURE] [OPERATING_SYSTEM]

Architectures: x86_64, aarch64, armv7, i386, auto
Operating Systems: linux, android, macos, freebsd, openbsd, netbsd, auto

Examples:
  $0 x86_64 linux
  $0 aarch64 android
  $0 armv7 linux
  $0 x86_64 freebsd
  $0 x86_64 macos
  $0 auto auto  # Auto-detect

Supported OS:
  - Linux (most distributions)
  - FreeBSD (and other BSD variants)
  - macOS/Darwin
  - Android (cross-compilation)

Note: For FreeBSD, ensure required packages are installed:
  sudo pkg install git gcc cmake curl

EOF
}

# Parse arguments
if [ $# -gt 0 ]; then
    case "$1" in
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            ARCH="$1"
            OS="${2:-auto}"
            ;;
    esac
fi

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    warning "This script should NOT be run as root/sudo!"
    warning "It will install software for the current user only."

    if prompt_yn "Continue as root anyway?" "N"; then
        warning "Installing as root user - this is not recommended"
    else
        info "Please run as normal user:"
        echo "  ./$(basename "$0")"
        exit 1
    fi
fi

# Initialize log
touch "$LOG_FILE"
echo "=== CARLA INSTALLER LOG - $(date) ===" > "$LOG_FILE"
echo "Target: ${OS}-${ARCH}" >> "$LOG_FILE"
echo "System: $(uname -a)" >> "$LOG_FILE"

# Run main
main "$@"
