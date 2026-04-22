pub fn generate() -> &'static str {
    r#"-- =============================================================================
-- options.lua — Neovim editor settings
--
-- These are plain vim.opt assignments.  Every option includes a comment
-- explaining what it does and why it's set this way.
-- =============================================================================

local opt = vim.opt

-- ── Appearance ────────────────────────────────────────────────────────────
opt.termguicolors  = true        -- enable 24-bit RGB colours in the terminal
opt.number         = true        -- show absolute line numbers
opt.relativenumber = true        -- show relative line numbers (great for jump motions)
opt.cursorline     = true        -- highlight the current line
opt.signcolumn     = "yes"       -- always show the sign column (prevents layout shift)
opt.colorcolumn    = "100"       -- soft ruler at 100 chars
opt.scrolloff      = 8           -- keep 8 lines visible above/below the cursor
opt.sidescrolloff  = 8
opt.wrap           = false       -- don't soft-wrap long lines
opt.showmode       = false       -- mode is shown by the statusline instead
opt.laststatus     = 3           -- single global statusline (Neovim 0.7+)

-- ── Indentation ───────────────────────────────────────────────────────────
opt.expandtab      = true        -- convert tabs to spaces
opt.shiftwidth     = 4           -- number of spaces per indent level
opt.tabstop        = 4           -- visual width of a tab character
opt.softtabstop    = 4
opt.smartindent    = true        -- auto-indent new lines intelligently

-- ── Search ────────────────────────────────────────────────────────────────
opt.ignorecase     = true        -- case-insensitive search …
opt.smartcase      = true        -- … unless the pattern contains uppercase
opt.hlsearch       = true        -- highlight all matches
opt.incsearch      = true        -- show matches as you type

-- ── Files & buffers ───────────────────────────────────────────────────────
opt.undofile       = true        -- persist undo history across sessions
opt.swapfile       = false       -- no swap files; rely on undo instead
opt.backup         = false
opt.fileencoding   = "utf-8"
opt.updatetime     = 250         -- faster CursorHold events (for LSP diagnostics)
opt.timeoutlen     = 300         -- time to wait for a key sequence to complete

-- ── Splits ────────────────────────────────────────────────────────────────
opt.splitright     = true        -- vertical splits open to the right
opt.splitbelow     = true        -- horizontal splits open below

-- ── Clipboard ─────────────────────────────────────────────────────────────
-- Sync Neovim's unnamed register with the system clipboard.
opt.clipboard      = "unnamedplus"

-- ── Completion popup ──────────────────────────────────────────────────────
opt.completeopt    = { "menu", "menuone", "noselect" }

-- ── Folding (Treesitter-based, if available) ──────────────────────────────
opt.foldmethod     = "expr"
opt.foldexpr       = "nvim_treesitter#foldexpr()"
opt.foldenable     = false       -- start with all folds open

-- ── Whitespace display ────────────────────────────────────────────────────
opt.list           = true
opt.listchars      = { tab = "» ", trail = "·", nbsp = "␣" }

-- ── Netrw (built-in explorer) disable ────────────────────────────────────
-- We use a dedicated explorer plugin, so disable netrw to avoid conflicts.
vim.g.loaded_netrw       = 1
vim.g.loaded_netrwPlugin = 1
"#
}
