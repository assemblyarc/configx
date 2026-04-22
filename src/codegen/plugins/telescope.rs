pub fn generate() -> &'static str {
    r#"-- =============================================================================
-- telescope.lua — fuzzy finder
--
-- Telescope is the de-facto standard for fuzzy-finding everything in Neovim:
-- files, grep results, buffers, help tags, git history, and more.
--
-- Key bindings (set in keymaps.lua):
--   Vim Purist  : <leader>ff  find files
--                 <leader>fg  live grep
--                 <leader>fb  buffers
--   VSCode-Like : <C-p>       quick open
--                 <C-f>       search in files
-- =============================================================================

return {
  {
    -- Core Telescope plugin
    "nvim-telescope/telescope.nvim",
    cmd          = "Telescope",
    dependencies = {
      "nvim-lua/plenary.nvim",
      -- Native FZF sorter — much faster than the Lua sorter for large repos
      { "nvim-telescope/telescope-fzf-native.nvim", build = "make" },
    },
    config = function()
      local telescope = require("telescope")
      local actions   = require("telescope.actions")

      telescope.setup({
        defaults = {
          -- Appearance
          prompt_prefix  = "  ",
          selection_caret = " ",
          path_display   = { "smart" },   -- shorten long paths intelligently
          layout_strategy = "horizontal",
          layout_config  = {
            horizontal = {
              width   = 0.9,
              preview_width = 0.5,
            },
          },
          border     = true,
          borderchars = { "─", "│", "─", "│", "╭", "╮", "╯", "╰" },

          -- Key mappings inside the picker
          mappings = {
            i = {
              ["<C-j>"]    = actions.move_selection_next,
              ["<C-k>"]    = actions.move_selection_previous,
              ["<C-q>"]    = actions.send_to_qflist + actions.open_qflist,
              ["<Esc>"]    = actions.close,
              ["<C-u>"]    = false,       -- clear prompt with <C-u>
            },
            n = {
              ["q"] = actions.close,
            },
          },

          -- Files and directories to always exclude
          file_ignore_patterns = {
            "node_modules/",
            "%.git/",
            "target/",          -- Rust build output
            "__pycache__/",
            "%.pyc",
          },
        },

        pickers = {
          find_files = {
            hidden = true,   -- include dotfiles in file search
          },
        },
      })

      -- Load the native FZF sorter for better performance
      telescope.load_extension("fzf")
    end,
  },
}
"#
}
