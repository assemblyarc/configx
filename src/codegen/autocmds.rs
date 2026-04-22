pub fn generate() -> &'static str {
    r#"-- =============================================================================
-- autocmds.lua — autocommands
--
-- Autocommands let Neovim react to events (opening a file, saving, etc.)
-- without requiring manual configuration.
-- =============================================================================

local augroup = vim.api.nvim_create_augroup
local autocmd = vim.api.nvim_create_autocmd

-- ── Highlight on yank ─────────────────────────────────────────────────────
-- Briefly flash the yanked region so you can confirm what was copied.
augroup("YankHighlight", { clear = true })
autocmd("TextYankPost", {
  group    = "YankHighlight",
  callback = function()
    vim.highlight.on_yank({ higroup = "IncSearch", timeout = 150 })
  end,
})

-- ── Restore cursor position ───────────────────────────────────────────────
-- Jump to the last known cursor position when reopening a file.
augroup("RestoreCursor", { clear = true })
autocmd("BufReadPost", {
  group   = "RestoreCursor",
  pattern = "*",
  callback = function()
    local mark = vim.api.nvim_buf_get_mark(0, '"')
    local lcount = vim.api.nvim_buf_line_count(0)
    if mark[1] > 0 and mark[1] <= lcount then
      pcall(vim.api.nvim_win_set_cursor, 0, mark)
    end
  end,
})

-- ── Close auxiliary windows with q ───────────────────────────────────────
-- Press q to close quickfix, help, and man page windows.
augroup("CloseWithQ", { clear = true })
autocmd("FileType", {
  group   = "CloseWithQ",
  pattern = { "qf", "help", "man", "notify", "lspinfo", "checkhealth" },
  callback = function(event)
    vim.bo[event.buf].buflisted = false
    vim.keymap.set("n", "q", "<cmd>close<CR>", { buffer = event.buf, silent = true })
  end,
})

-- ── Auto-resize splits on terminal resize ─────────────────────────────────
augroup("ResizeSplits", { clear = true })
autocmd("VimResized", {
  group    = "ResizeSplits",
  callback = function()
    local current_tab = vim.fn.tabpagenr()
    vim.cmd("tabdo wincmd =")
    vim.cmd("tabnext " .. current_tab)
  end,
})

-- ── Strip trailing whitespace on save ─────────────────────────────────────
augroup("TrimWhitespace", { clear = true })
autocmd("BufWritePre", {
  group   = "TrimWhitespace",
  pattern = "*",
  callback = function()
    local save = vim.fn.winsaveview()
    vim.cmd([[keeppatterns %s/\s\+$//e]])
    vim.fn.winrestview(save)
  end,
})
"#
}
