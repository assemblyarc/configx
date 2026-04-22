pub fn generate() -> &'static str {
    r#"-- =============================================================================
-- statusline.lua — lualine statusline
--
-- lualine provides a fast, configurable statusline with built-in sections
-- for mode, git branch, filename, diagnostics, filetype, and cursor position.
-- =============================================================================

return {
  "nvim-lualine/lualine.nvim",
  event        = "VimEnter",
  dependencies = { "nvim-tree/nvim-web-devicons" },
  config = function()
    require("lualine").setup({
      options = {
        -- Choose a theme from: auto, dracula, gruvbox, onedark, tokyonight, ...
        -- "auto" picks the theme based on the active colorscheme.
        theme            = "auto",
        globalstatus     = true,     -- single statusline across all windows
        component_separators = { left = "", right = "" },
        section_separators  = { left = "", right = "" },
        disabled_filetypes  = { statusline = { "dashboard", "alpha", "starter" } },
      },

      sections = {
        -- Left side
        lualine_a = { "mode" },
        lualine_b = {
          { "branch", icon = "" },
          {
            "diff",
            symbols = { added = " ", modified = " ", removed = " " },
          },
        },
        lualine_c = {
          {
            "filename",
            path = 1,         -- show relative path
            symbols = {
              modified = " ●",
              readonly = " ",
              unnamed  = "[No Name]",
            },
          },
        },

        -- Right side
        lualine_x = {
          {
            "diagnostics",
            sources  = { "nvim_lsp" },
            sections = { "error", "warn", "info", "hint" },
            symbols  = { error = " ", warn = " ", info = " ", hint = "󰝶 " },
          },
        },
        lualine_y = { "filetype" },
        lualine_z = { "location", "progress" },
      },

      inactive_sections = {
        lualine_c = { "filename" },
        lualine_x = { "location" },
      },

      -- Extensions add context-aware sections for specific plugins
      extensions = { "neo-tree", "nvim-tree", "oil", "lazy", "quickfix" },
    })
  end,
}
"#
}
