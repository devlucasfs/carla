#!/bin/bash
#
# Carla installer
# Builds cnorn, Morgana, Carla, Norn, Eva, Runa and Samar from source
# and installs everything under ~/.carla

set -euo pipefail
IFS=$'\n\t'

# ----------------------------------------------------------------------
#  CONFIG
# ----------------------------------------------------------------------
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

LOG_FILE="${HOME}/.carla_installer.log"
INSTALL_TIMESTAMP=$(date '+%Y%m%d_%H%M%S')

INSTALL_BASE="${HOME}/.carla"
INSTALL_BIN="${INSTALL_BASE}/bin"
INSTALL_STD="${INSTALL_BASE}/std"
INSTALL_EXTENSORS="${INSTALL_BASE}/extensors"

REPO_CNORN="https://github.com/devlucasfs/cnorn.git"
REPO_MORGANA="https://github.com/devlucasfs/morgana.git"
REPO_CARLA="https://github.com/devlucasfs/carla.git"
REPO_NORN="https://github.com/devlucasfs/norn.git"
REPO_EVA="https://github.com/carla-repos/eva.git"
REPO_RUNA="https://github.com/devlucasfs/runa.git"
REPO_SAMAR="https://github.com/devlucasfs/samar.git"
REPO_STD="https://github.com/Carla-repos/carla-std.git"

TEMP_DIR=""

# ----------------------------------------------------------------------
#  LOGGING
# ----------------------------------------------------------------------
log() {
    local color="$1"
    local message="$2"
    echo -e "${color}${message}${NC}" >&2
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] ${message}" >> "$LOG_FILE"
}

info()    { log "$BLUE"   "INFO: $1"; }
success() { log "$GREEN"  "SUCCESS: $1"; }
warning() { log "$YELLOW" "WARNING: $1"; }
error()   { log "$RED"    "ERROR: $1"; }

cleanup() {
    local code=$?
    if [ "$code" -ne 0 ]; then
        error "Installation failed! Check log: $LOG_FILE"
    fi
    if [ -n "$TEMP_DIR" ] && [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
    fi
}
trap cleanup EXIT

# ----------------------------------------------------------------------
#  SYSTEM CHECKS
# ----------------------------------------------------------------------
detect_package_manager() {
    if command -v apt >/dev/null 2>&1; then echo "apt"
    elif command -v dnf >/dev/null 2>&1; then echo "dnf"
    elif command -v yum >/dev/null 2>&1; then echo "yum"
    elif command -v pacman >/dev/null 2>&1; then echo "pacman"
    elif command -v brew >/dev/null 2>&1; then echo "brew"
    elif command -v pkg >/dev/null 2>&1; then echo "pkg"
    else echo "unknown"
    fi
}

check_commands() {
    # command -> hint shown to the user
    local required=(git g++ cargo go ocamlc perl)
    local missing=()

    for cmd in "${required[@]}"; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            missing+=("$cmd")
        fi
    done

    if [ ${#missing[@]} -gt 0 ]; then
        error "Missing required commands: ${missing[*]}"
        info "Install them with your package manager. Examples:"
        case "$(detect_package_manager)" in
            apt)    echo "  sudo apt install git g++ golang ocaml perl && curl https://sh.rustup.rs -sSf | sh" ;;
            dnf|yum) echo "  sudo dnf install git gcc-c++ golang ocaml perl cargo" ;;
            pacman) echo "  sudo pacman -S git gcc go ocaml perl rust" ;;
            brew)   echo "  brew install git gcc go ocaml perl rust" ;;
            pkg)    echo "  sudo pkg install git gcc go ocaml perl5 rust" ;;
            *)      echo "  Please install the missing tools manually" ;;
        esac
        return 1
    fi

    success "All required commands are available"
}

check_disk_space() {
    local required_mb=1000
    local available_kb
    available_kb=$(df -Pk "$HOME" | awk 'NR==2 {print $4}')
    local available_mb=$((available_kb / 1024))

    if [ "$available_mb" -lt "$required_mb" ]; then
        error "Insufficient disk space. Required: ${required_mb}MB, Available: ${available_mb}MB"
        return 1
    fi
    success "Disk space check passed: ${available_mb}MB available"
}

# ----------------------------------------------------------------------
#  SHELL / PATH
# ----------------------------------------------------------------------
get_shell_rc() {
    case "$(basename "${SHELL:-/bin/sh}")" in
        bash) echo "$HOME/.bashrc" ;;
        zsh)  echo "${ZDOTDIR:-$HOME}/.zshrc" ;;
        fish) echo "$HOME/.config/fish/config.fish" ;;
        *)    echo "$HOME/.profile" ;;
    esac
}

add_to_path() {
    local dir="$1"
    local shell_rc
    shell_rc=$(get_shell_rc)

    export PATH="${dir}:$PATH"

    mkdir -p "$(dirname "$shell_rc")"
    touch "$shell_rc"

    if grep -qF "# Added by Carla installer" "$shell_rc" 2>/dev/null; then
        info "PATH already configured in ${shell_rc}"
        return 0
    fi

    {
        echo ""
        echo "# Added by Carla installer - ${INSTALL_TIMESTAMP}"
        if [ "$(basename "${SHELL:-}")" = "fish" ]; then
            echo "set -gx PATH \"${dir}\" \$PATH"
            echo "set -gx LD_LIBRARY_PATH \"${dir}\" \$LD_LIBRARY_PATH"
        else
            echo "export PATH=\"${dir}:\$PATH\""
            echo "export LD_LIBRARY_PATH=\"${dir}:\${LD_LIBRARY_PATH:-}\""
            echo "export DYLD_LIBRARY_PATH=\"${dir}:\${DYLD_LIBRARY_PATH:-}\""
        fi
    } >> "$shell_rc"

    success "Added ${dir} to PATH in ${shell_rc}"
}

# ----------------------------------------------------------------------
#  HELPERS
# ----------------------------------------------------------------------
clone_repo() {
    local url="$1"
    local name="$2"

    info "Cloning ${name}..."
    if ! git clone --depth 1 "$url" "${TEMP_DIR}/${name}" >>"$LOG_FILE" 2>&1; then
        error "Failed to clone ${name} (${url})"
        return 1
    fi
}

# Runs a command inside a repo directory, logging all output
run_in() {
    local dir="$1"; shift
    ( cd "$dir" && "$@" ) >>"$LOG_FILE" 2>&1
}

# Copies the first file matching a name found under a directory
install_artifact() {
    local search_dir="$1"
    local file_name="$2"
    local dest="$3"
    local found

    found=$(find "$search_dir" -type f -name "$file_name" -not -path "*/deps/*" 2>/dev/null | head -1)

    if [ -z "$found" ]; then
        error "Could not find ${file_name} in ${search_dir}"
        return 1
    fi

    cp "$found" "$dest"
    success "Installed ${file_name} to ${dest}"
}

# ----------------------------------------------------------------------
#  INSTALLERS
# ----------------------------------------------------------------------

# cnorn (OCaml). Must come first: Morgana needs it to build.
install_cnorn() {
    info "Installing cnorn..."
    clone_repo "$REPO_CNORN" "cnorn" || return 1

    local dir="${TEMP_DIR}/cnorn"
    if [ ! -f "${dir}/cnorn.ml" ]; then
        error "cnorn.ml not found in the cnorn repository"
        return 1
    fi

    if ! run_in "$dir" ocamlc -o "${INSTALL_BIN}/cnorn" cnorn.ml; then
        error "cnorn build failed - check log: $LOG_FILE"
        return 1
    fi

    chmod +x "${INSTALL_BIN}/cnorn"
    success "cnorn installed to ${INSTALL_BIN}"
}

# Morgana (perl build.pl -> bin/libmorgana.so) + Sake config
install_morgana() {
    info "Installing Morgana..."
    clone_repo "$REPO_MORGANA" "morgana" || return 1

    local dir="${TEMP_DIR}/morgana"
    if ! run_in "$dir" perl build.pl; then
        error "Morgana build failed - check log: $LOG_FILE"
        return 1
    fi

    install_artifact "${dir}/bin" "libmorgana.so" "$INSTALL_BIN/" || return 1

    if [ -f "${dir}/Sake" ]; then
        cp "${dir}/Sake" "${INSTALL_BASE}/Basic"
        success "Copied Sake to ${INSTALL_BASE}/Basic"
    else
        warning "Sake file not found in the Morgana repository"
    fi
}

# Carla (Rust)
install_carla() {
    info "Installing Carla..."
    clone_repo "$REPO_CARLA" "carla" || return 1

    local dir="${TEMP_DIR}/carla"
    if ! run_in "$dir" cargo build --release; then
        error "Carla build failed - check log: $LOG_FILE"
        return 1
    fi

    cp "${dir}/target/release/carla" "${INSTALL_BIN}/carla"
    chmod +x "${INSTALL_BIN}/carla"
    success "Carla installed to ${INSTALL_BIN}"
}

# Norn (Go)
install_norn() {
    info "Installing Norn..."
    clone_repo "$REPO_NORN" "norn" || return 1

    local dir="${TEMP_DIR}/norn"
    if ! run_in "$dir" go build -o "${INSTALL_BIN}/norn" .; then
        error "Norn build failed - check log: $LOG_FILE"
        return 1
    fi

    chmod +x "${INSTALL_BIN}/norn"
    success "Norn installed to ${INSTALL_BIN}"
}

# Eva and Runa (Rust libraries)
install_rust_lib() {
    local name="$1"
    local url="$2"

    info "Installing ${name}..."
    clone_repo "$url" "$name" || return 1

    local dir="${TEMP_DIR}/${name}"

    # Cargo.toml may not be at the repo root: pick the shallowest one
    local manifest
    manifest=$(find "$dir" -maxdepth 3 -name Cargo.toml -not -path "*/target/*" 2>/dev/null \
        | awk '{ print gsub("/","/") " " $0 }' | sort -n | head -1 | cut -d' ' -f2-)

    if [ -z "$manifest" ]; then
        error "No Cargo.toml found in ${name} repository"
        info "Repository contents:"
        ls -la "$dir" >&2
        return 1
    fi
    info "Using manifest: ${manifest#"$dir"/}"

    if ! run_in "$dir" cargo build --release --manifest-path "$manifest"; then
        error "${name} build failed - check log: $LOG_FILE"
        return 1
    fi

    local lib
    lib=$(find "$dir" -type f -name "lib${name}.so" -path "*/release/*" -not -path "*/deps/*" 2>/dev/null | head -1)

    if [ -z "$lib" ]; then
        error "lib${name}.so not found after build (is crate-type set to cdylib?)"
        info "Built artifacts:"
        find "$dir" -path "*/release/*" -maxdepth 6 -type f \( -name "*.so" -o -name "*.a" \) >&2
        return 1
    fi

    cp "$lib" "${INSTALL_BIN}/"
    success "Installed lib${name}.so to ${INSTALL_BIN}"
}

# Samar (perl build.pl, same scheme as Morgana). Needs libeva.so in ~/.carla/bin.
install_samar() {
    info "Installing Samar..."
    clone_repo "$REPO_SAMAR" "samar" || return 1

    local dir="${TEMP_DIR}/samar"
    if ! run_in "$dir" perl build.pl; then
        error "Samar build failed - check log: $LOG_FILE"
        return 1
    fi

    install_artifact "${dir}/bin" "libsamar.so" "$INSTALL_BIN/" || return 1
}

install_std() {
    info "Installing STD..."
    clone_repo "$REPO_STD" "carla-std" || return 1

    rm -rf "$INSTALL_STD"
    cp -r "${TEMP_DIR}/carla-std" "$INSTALL_STD"
    rm -rf "${INSTALL_STD}/.git"
    success "STD installed to ${INSTALL_STD}"
}

# ----------------------------------------------------------------------
#  VERIFICATION
# ----------------------------------------------------------------------
verify_installation() {
    info "Verifying installation..."
    local ok=true

    local files=(
        "${INSTALL_BIN}/cnorn"
        "${INSTALL_BIN}/carla"
        "${INSTALL_BIN}/norn"
        "${INSTALL_BIN}/libmorgana.so"
        "${INSTALL_BIN}/libeva.so"
        "${INSTALL_BIN}/libruna.so"
        "${INSTALL_BIN}/libsamar.so"
        "${INSTALL_BASE}/Basic"
    )

    for f in "${files[@]}"; do
        if [ -e "$f" ]; then
            success "✓ ${f}"
        else
            warning "✗ Missing: ${f}"
            ok=false
        fi
    done

    if [ -d "$INSTALL_STD" ]; then
        success "✓ ${INSTALL_STD}"
    else
        warning "✗ Missing: ${INSTALL_STD}"
        ok=false
    fi

    if [ "$ok" = true ]; then
        success "All components verified successfully"
    else
        warning "Some components failed verification"
    fi
}

# ----------------------------------------------------------------------
#  PROMPTS
# ----------------------------------------------------------------------
prompt_yn() {
    local msg="$1"
    local default="${2:-Y}"
    local hint="[Y/n]"
    [ "$default" = "N" ] && hint="[y/N]"

    while true; do
        echo -n "$msg $hint: " >&2
        read -r choice
        choice="${choice:-$default}"
        case "${choice,,}" in
            y|yes) return 0 ;;
            n|no)  return 1 ;;
            *) echo "Please answer 'y' or 'n'" >&2 ;;
        esac
    done
}

show_usage() {
    cat << EOF
Usage: $0 [-h|--help]

Builds and installs the Carla toolchain into ${INSTALL_BASE}.

Required tools:
  git, g++, rust/cargo, golang, ocaml (ocamlc), perl

Components (in install order):
  cnorn    OCaml      -> ~/.carla/bin/cnorn
  eva      Rust       -> ~/.carla/bin/libeva.so
  runa     Rust       -> ~/.carla/bin/libruna.so
  samar    perl       -> ~/.carla/bin/libsamar.so
  morgana  perl       -> ~/.carla/bin/libmorgana.so (+ ~/.carla/Basic)
  carla    Rust       -> ~/.carla/bin/carla
  norn     Go         -> ~/.carla/bin/norn
  std                 -> ~/.carla/std
EOF
}

# ----------------------------------------------------------------------
#  MAIN
# ----------------------------------------------------------------------
main() {
    echo -e "${BLUE}"
    cat << "EOF"
    ___          _                     _   __  __
   / __|__ _ _ _| |__ _   __ _ _ _  __| | |  \/  |___ _ _ __ _ __ _ _ _  __ _
  | (__/ _` | '_| / _` | / _` | ' \/ _` | | |\/| / _ \ '_/ _` / _` | ' \/ _` |
   \___\__,_|_| |_\__,_| \__,_|_||_\__,_| |_|  |_\___/_| \__, \__,_|_||_\__,_|
EOF
    echo -e "${NC}"

    info "Carla installer"
    info "Install directory: ${INSTALL_BASE}"
    info "Log file: ${LOG_FILE}"

    check_commands || exit 1
    check_disk_space || exit 1

    if [ -d "$INSTALL_BASE" ]; then
        warning "Existing installation found at $INSTALL_BASE"
        if prompt_yn "Remove and reinstall?" "Y"; then
            rm -rf "$INSTALL_BASE"
            info "Removed old installation"
        else
            info "Installation cancelled by user"
            exit 0
        fi
    fi

    mkdir -p "$INSTALL_BIN" "$INSTALL_EXTENSORS"

    TEMP_DIR=$(mktemp -d)
    info "Temporary directory: $TEMP_DIR"

    # Builds need our own binaries/libs (cnorn, libeva.so) visible
    export PATH="${INSTALL_BIN}:${PATH}"
    export LD_LIBRARY_PATH="${INSTALL_BIN}:${LD_LIBRARY_PATH:-}"
    export DYLD_LIBRARY_PATH="${INSTALL_BIN}:${DYLD_LIBRARY_PATH:-}"

    install_cnorn                         || exit 1   # required by Morgana
    install_rust_lib "eva"  "$REPO_EVA"   || exit 1   # required by Samar
    install_rust_lib "runa" "$REPO_RUNA"  || exit 1
    install_samar                         || exit 1   # required by Morgana
    install_morgana                       || exit 1
    install_carla                         || exit 1
    install_norn                          || exit 1
    install_std                           || exit 1

    add_to_path "$INSTALL_BIN"
    verify_installation

    echo
    success "🎉 Installation completed successfully!"
    echo
    info "Next steps:"
    echo "  1. Run: source $(get_shell_rc)"
    echo "  2. Or simply restart your terminal"
    echo "  3. Test with: carla version"
    echo
    info "Log file: $LOG_FILE"
}

# ----------------------------------------------------------------------
#  ENTRY POINT
# ----------------------------------------------------------------------
case "${1:-}" in
    -h|--help)
        show_usage
        exit 0
        ;;
esac

if [ "$EUID" -eq 0 ]; then
    warning "This script should NOT be run as root/sudo!"
    warning "It will install software for the current user only."
    if ! prompt_yn "Continue as root anyway?" "N"; then
        info "Please run as a normal user: ./$(basename "$0")"
        exit 1
    fi
fi

echo "=== CARLA INSTALLER LOG - $(date) ===" > "$LOG_FILE"
echo "System: $(uname -a)" >> "$LOG_FILE"

main
