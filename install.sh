set -euo pipefail

BOLD='\033[1m'
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
RESET='\033[0m'

info()    { echo -e "${CYAN}==>${RESET} ${BOLD}$*${RESET}"; }
success() { echo -e "${GREEN}  ✓${RESET} $*"; }
warn()    { echo -e "${YELLOW}  !${RESET} $*"; }
error()   { echo -e "${RED}  ✗${RESET} $* " >&2; exit 1; }

INSTALL_DIR=""
VERSION="latest"
NO_CONFIRM=false
DRY_RUN=false

while [[ $# -gt 0 ]]; do
  case $1 in
    --to)       INSTALL_DIR="$2"; shift 2 ;;
    --version)  VERSION="$2";     shift 2 ;;
    --no-confirm) NO_CONFIRM=true; shift ;;
    --dry-run)  DRY_RUN=true;     shift ;;
    *)          error "Unknown argument: $1" ;;
  esac
done

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64)  PLATFORM="aarch64-apple-darwin" ;;
      x86_64) PLATFORM="x86_64-apple-darwin"  ;;
      *)      error "Unsupported macOS architecture: $ARCH" ;;
    esac
    ;;
  Linux)
    case "$ARCH" in
      x86_64)          PLATFORM="x86_64-linux"  ;;
      aarch64|arm64)   PLATFORM="aarch64-linux" ;;
      *)               error "Unsupported Linux architecture: $ARCH" ;;
    esac
    ;;
  *)
    error "Unsupported OS: $OS. Please build from source: cargo install --git https://github.com/assemblyarc/configx"
    ;;
esac

ARCHIVE="configx-${PLATFORM}.tar.gz"


REPO="assemblyarc/configx"
BASE_URL="https://github.com/${REPO}/releases"

if [[ "$VERSION" == "latest" ]]; then
  info "Fetching latest release version..."
  VERSION="$(curl -fsSL "${BASE_URL}/latest" | grep -o 'tag/v[^"]*' | head -1 | cut -d/ -f2)"
  [[ -n "$VERSION" ]] || error "Could not determine latest version. Check https://github.com/${REPO}/releases"
fi

DOWNLOAD_URL="${BASE_URL}/download/${VERSION}/${ARCHIVE}"


if [[ -z "$INSTALL_DIR" ]]; then
  if [[ -w "/usr/local/bin" ]]; then
    INSTALL_DIR="/usr/local/bin"
  else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    # Warn if not in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
      warn "$INSTALL_DIR is not in your PATH. Add this to your shell profile:"
      warn "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
  fi
fi

echo ""
echo -e "  ${BOLD}configx${RESET} — Neovim Config Wizard"
echo ""
echo -e "  Version  : ${CYAN}${VERSION}${RESET}"
echo -e "  Platform : ${CYAN}${PLATFORM}${RESET}"
echo -e "  Install  : ${CYAN}${INSTALL_DIR}/configx${RESET}"
echo ""

if $DRY_RUN; then
  info "Dry run — would download: $DOWNLOAD_URL"
  info "Dry run — would install to: ${INSTALL_DIR}/configx"
  exit 0
fi

if ! $NO_CONFIRM; then
  read -rp "Continue? [Y/n] " answer
  case "$answer" in
    [Nn]*) echo "Aborted."; exit 0 ;;
  esac
fi


TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

info "Downloading ${ARCHIVE}..."
curl -fsSL --progress-bar "$DOWNLOAD_URL" -o "${TMP_DIR}/${ARCHIVE}"

SHA_URL="${BASE_URL}/download/${VERSION}/${ARCHIVE}.sha256"
if curl -fsSL "$SHA_URL" -o "${TMP_DIR}/${ARCHIVE}.sha256" 2>/dev/null; then
  info "Verifying checksum..."
  if command -v sha256sum &>/dev/null; then
    (cd "$TMP_DIR" && sha256sum --check "${ARCHIVE}.sha256" --quiet)
  else
    EXPECTED="$(awk '{print $1}' "${TMP_DIR}/${ARCHIVE}.sha256")"
    ACTUAL="$(shasum -a 256 "${TMP_DIR}/${ARCHIVE}" | awk '{print $1}')"
    [[ "$EXPECTED" == "$ACTUAL" ]] || error "Checksum mismatch! Download may be corrupted."
  fi
  success "Checksum verified"
fi

info "Extracting..."
tar -xzf "${TMP_DIR}/${ARCHIVE}" -C "$TMP_DIR"

info "Installing to ${INSTALL_DIR}/configx..."
install -m 755 "${TMP_DIR}/configx" "${INSTALL_DIR}/configx"

success "configx ${VERSION} installed!"
echo ""
echo -e "  Run: ${BOLD}configx${RESET}"
echo -e "  Or:  ${BOLD}configx --help${RESET}"
echo ""
