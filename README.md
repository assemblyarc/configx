# configx

> An offline, single-binary TUI wizard that generates a complete, modular,
> well-commented Neovim config — no browser, no templates to clone, no fuss.

```
⚙  configx · Neovim Config Wizard                      [1/7]
╭──────────────────────────────────────────────────────────╮
│  Step 1 — Languages                                      │
├──────────────────────────────────────────────────────────┤
│  ▶ ●  Rust                                               │
│    ○  Python                                             │
│    ●  TypeScript                                         │
│    ○  Go                                                 │
╰──────────────────────────────────────────────────────────╯
  ↑↓ navigate  ·  Space toggle  ·  Enter next  ·  q quit
```

## Install

<!-- ### Homebrew (macOS + Linux)

```bash
brew install configx
``` -->

### cargo install

```bash
cargo install configx
```

### curl one-liner (no Rust or Homebrew needed)

```bash
curl -fsSL https://raw.githubusercontent.com/assemblyarc/configx/main/install.sh | bash
```

Install to a custom directory:

```bash
curl -fsSL https://raw.githubusercontent.com/assemblyarc/configx/main/install.sh | bash -s -- --to ~/.local/bin
```

### Manual download

Grab the binary for your platform from the
[Releases page](https://github.com/assemblyarc/configx/releases):

| Platform | File |
|----------|------|
| macOS Apple Silicon | `configx-aarch64-apple-darwin.tar.gz` |
| macOS Intel | `configx-x86_64-apple-darwin.tar.gz` |
| Linux x86_64 | `configx-x86_64-linux.tar.gz` |
| Linux arm64 | `configx-aarch64-linux.tar.gz` |

```bash
tar -xzf configx-*.tar.gz
sudo mv configx /usr/local/bin/
```

## Usage

```bash
# Interactive TUI wizard — generates ./nvim-output/ by default
configx

# Preview generated Lua without writing to disk
configx --dry-run

# Write to a specific directory
configx --output ~/Desktop/my-nvim

# Specific version
configx --version
```

### After generating

```bash
# Back up existing config
mv ~/.config/nvim ~/.config/nvim.bak

# Drop the generated config in
cp -r ./nvim-output ~/.config/nvim

# Open Neovim — lazy.nvim installs everything on first launch
nvim
```

## What gets generated

```
~/.config/nvim/
├── init.lua                    ← lazy.nvim bootstrap
└── lua/
    ├── config/
    │   ├── options.lua         ← vim.opt settings (commented)
    │   ├── keymaps.lua         ← style-aware key bindings
    │   └── autocmds.lua        ← yank highlight, cursor restore, …
    └── plugins/
        ├── colorscheme.lua     ← always present
        ├── treesitter.lua      ← if Treesitter selected
        ├── lsp.lua             ← mason + lspconfig + conform
        ├── completion.lua      ← nvim-cmp + LuaSnip
        ├── explorer.lua        ← neo-tree / nvim-tree / oil
        ├── telescope.lua       ← if selected
        ├── git.lua             ← gitsigns + neogit
        ├── statusline.lua      ← lualine
        └── extras.lua          ← which-key, autopairs, todo-comments, …
```

## Wizard steps

| Step | What you pick |
|------|---------------|
| Languages | Rust, Python, TypeScript, Go, C/C++, Lua, Java, Zig |
| Features | LSP, Completion (nvim-cmp), Snippets (LuaSnip), Treesitter |
| Colorscheme | Catppuccin, Tokyo Night, Gruvbox, Rose Pine, Nord, OneDark |
| File Explorer | neo-tree, nvim-tree, oil.nvim, none |
| Keybinding Style | Vim Purist or VSCode-Like |
| Extra Plugins | Telescope, which-key, gitsigns, Neogit, lualine, autopairs, … |

## Key bindings in the wizard

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `Space` | Toggle item (multi-select) |
| `Enter` / `→` | Advance / select |
| `←` / `Backspace` | Go back |
| `g` | Generate (on confirm screen) |
| `q` / `Ctrl-C` | Quit |

## Build from source

```bash
git clone https://github.com/assemblyarc/configx
cd configx
cargo build --release
./target/release/configx
```

Requires Rust 1.85+ (edition 2024).

## License

MIT — see [LICENSE](LICENSE).
