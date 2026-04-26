# Distributing configx

This directory contains everything needed to distribute configx via:
- **Homebrew** (macOS + Linux)
- **curl one-liner** (any Unix)
- **cargo install** (Rust users)
- **GitHub Releases** (manual download)

## First release checklist

1. Push a tag: `git tag v0.1.1 && git push --tags`
2. GitHub Actions builds and uploads binaries automatically
3. Update `Formula/configx.rb` SHA256 checksums (printed by the release workflow)
4. Push the formula to your tap: `assemblyarc/homebrew-tap`
