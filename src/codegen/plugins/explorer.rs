use crate::wizard::state::{Explorer, WizardState};

pub fn generate(state: &WizardState) -> Option<String> {
    match state.explorer {
        Explorer::NeoTree => Some(neo_tree()),
        Explorer::NvimTree => Some(nvim_tree()),
        Explorer::Oil => Some(oil()),
        Explorer::None => None,
    }
}

fn neo_tree() -> String {
    r#"-- =============================================================================
-- explorer.lua — file explorer (neo-tree)
--
-- neo-tree provides a sidebar file tree with icons, git status, and
-- buffers/document symbols panels.  Requires a Nerd Font in your terminal.
--
-- Keybindings: <leader>e to toggle
-- =============================================================================

return {
  "nvim-neo-tree/neo-tree.nvim",
  branch       = "v3.x",
  cmd          = "Neotree",
  dependencies = {
    "nvim-lua/plenary.nvim",       -- utility library
    "nvim-tree/nvim-web-devicons", -- file icons (needs Nerd Font)
    "MunifTanjim/nui.nvim",        -- UI component library
  },
  config = function()
    -- Disable netrw before neo-tree loads (already set in options.lua, but
    -- setting it here too ensures there's no race condition).
    vim.g.loaded_netrw       = 1
    vim.g.loaded_netrwPlugin = 1

    require("neo-tree").setup({
      close_if_last_window = true,   -- close Neovim if neo-tree is the last window
      popup_border_style   = "rounded",

      window = {
        position = "left",
        width    = 35,
        mappings = {
          ["<space>"] = "none",    -- don't hijack leader in the tree
        },
      },

      filesystem = {
        follow_current_file   = { enabled = true },  -- reveal the active file
        use_libuv_file_watcher = true,               -- auto-refresh on disk changes
        filtered_items = {
          visible         = false,
          hide_dotfiles   = false,  -- show dotfiles like .gitignore
          hide_gitignored = true,
        },
      },

      -- Git status icons in the tree
      git_status = {
        symbols = {
          added     = "✚",
          modified  = "",
          deleted   = "✖",
          renamed   = "➜",
          untracked = "★",
          ignored   = "◌",
          unstaged  = "✗",
          staged    = "✓",
          conflict  = "",
        },
      },
    })
  end,
}
"#
    .to_string()
}

fn nvim_tree() -> String {
    r#"-- =============================================================================
-- explorer.lua — file explorer (nvim-tree)
--
-- Classic Vim-style sidebar file explorer.  Lightweight and fast.
--
-- Keybindings: <leader>e to toggle
-- =============================================================================

return {
  "nvim-tree/nvim-tree.lua",
  cmd          = { "NvimTreeToggle", "NvimTreeFocus" },
  dependencies = { "nvim-tree/nvim-web-devicons" },
  config = function()
    require("nvim-tree").setup({
      sort = { sorter = "case_sensitive" },
      view = { width = 35 },
      renderer = {
        group_empty = true,   -- collapse single-child directories
        icons = {
          git_placement = "before",
        },
      },
      filters = {
        dotfiles = false,   -- show dotfiles
      },
      git = {
        enable  = true,
        ignore  = false,
        timeout = 400,
      },
      actions = {
        open_file = {
          quit_on_open = false,  -- keep the tree open after opening a file
        },
      },
    })
  end,
}
"#
    .to_string()
}

fn oil() -> String {
    r#"-- =============================================================================
-- explorer.lua — file explorer (oil.nvim)
--
-- oil.nvim lets you navigate and edit the filesystem like a normal Neovim
-- buffer — rename, delete, or create files by editing text.
--
-- Keybindings: <leader>e to open, - to go up a directory
-- =============================================================================

return {
  "stevearc/oil.nvim",
  cmd          = "Oil",
  dependencies = { "nvim-tree/nvim-web-devicons" },
  config = function()
    require("oil").setup({
      -- Show the current directory path as the buffer name
      default_file_explorer = true,

      columns = {
        "icon",
        "permissions",
        "size",
        "mtime",
      },

      -- Key mappings inside an oil buffer
      keymaps = {
        ["g?"] = "actions.show_help",
        ["<CR>"] = "actions.select",
        ["<C-v>"] = "actions.select_vsplit",
        ["<C-s>"] = "actions.select_split",
        ["-"]    = "actions.parent",
        ["_"]    = "actions.open_cwd",
        ["gs"]   = "actions.change_sort",
        ["gx"]   = "actions.open_external",
        ["g."]   = "actions.toggle_hidden",
        ["<C-c>"] = "actions.close",
        ["<C-l>"] = "actions.refresh",
      },

      view_options = {
        show_hidden = true,   -- show dotfiles by default
      },

      float = {
        padding      = 2,
        max_width    = 90,
        max_height   = 0,
        border       = "rounded",
        win_options  = { winblend = 0 },
      },
    })
  end,
}
"#
    .to_string()
}
