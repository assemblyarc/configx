use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum PluginCategory {
    Lsp,
    Completion,
    Treesitter,
    Explorer,
    Ui,
    Editor,
    Git,
    Telescope,
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub repo: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: PluginCategory,
    pub event: Option<&'static str>,
    pub cmd: Option<&'static str>,
    pub ft: Option<&'static str>,
}

pub const PLUGIN_MASON: Plugin = Plugin {
    repo: "williamboman/mason.nvim",
    name: "mason",
    description: "Portable package manager for LSP servers, DAPs, linters, and formatters",
    category: PluginCategory::Lsp,
    event: None,
    cmd: Some("Mason"),
    ft: None,
};

pub const PLUGIN_MASON_LSPCONFIG: Plugin = Plugin {
    repo: "williamboman/mason-lspconfig.nvim",
    name: "mason-lspconfig",
    description: "Bridges mason.nvim with nvim-lspconfig",
    category: PluginCategory::Lsp,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_NVIM_LSPCONFIG: Plugin = Plugin {
    repo: "neovim/nvim-lspconfig",
    name: "nvim-lspconfig",
    description: "Quickstart configs for Neovim's built-in LSP client",
    category: PluginCategory::Lsp,
    event: Some("BufReadPre"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_CONFORM: Plugin = Plugin {
    repo: "stevearc/conform.nvim",
    name: "conform",
    description: "Lightweight yet powerful formatter plugin for Neovim",
    category: PluginCategory::Lsp,
    event: Some("BufWritePre"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_NVIM_CMP: Plugin = Plugin {
    repo: "hrsh7th/nvim-cmp",
    name: "nvim-cmp",
    description: "A completion plugin for Neovim coded in Lua",
    category: PluginCategory::Completion,
    event: Some("InsertEnter"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_CMP_NVIM_LSP: Plugin = Plugin {
    repo: "hrsh7th/cmp-nvim-lsp",
    name: "cmp-nvim-lsp",
    description: "nvim-cmp source for Neovim's built-in LSP",
    category: PluginCategory::Completion,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_CMP_BUFFER: Plugin = Plugin {
    repo: "hrsh7th/cmp-buffer",
    name: "cmp-buffer",
    description: "nvim-cmp source for buffer words",
    category: PluginCategory::Completion,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_CMP_PATH: Plugin = Plugin {
    repo: "hrsh7th/cmp-path",
    name: "cmp-path",
    description: "nvim-cmp source for filesystem paths",
    category: PluginCategory::Completion,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_LUASNIP: Plugin = Plugin {
    repo: "L3MON4D3/LuaSnip",
    name: "LuaSnip",
    description: "Snippet engine for Neovim written in Lua",
    category: PluginCategory::Completion,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_CMP_LUASNIP: Plugin = Plugin {
    repo: "saadparwaiz1/cmp_luasnip",
    name: "cmp_luasnip",
    description: "LuaSnip completion source for nvim-cmp",
    category: PluginCategory::Completion,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_FRIENDLY_SNIPPETS: Plugin = Plugin {
    repo: "rafamadriz/friendly-snippets",
    name: "friendly-snippets",
    description: "Set of preconfigured snippets for different languages",
    category: PluginCategory::Completion,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_TREESITTER: Plugin = Plugin {
    repo: "nvim-treesitter/nvim-treesitter",
    name: "nvim-treesitter",
    description: "Neovim Treesitter configurations and abstraction layer",
    category: PluginCategory::Treesitter,
    event: Some("BufReadPost"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_TREESITTER_TEXTOBJECTS: Plugin = Plugin {
    repo: "nvim-treesitter/nvim-treesitter-textobjects",
    name: "nvim-treesitter-textobjects",
    description: "Syntax-aware text objects via Treesitter",
    category: PluginCategory::Treesitter,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_NEO_TREE: Plugin = Plugin {
    repo: "nvim-neo-tree/neo-tree.nvim",
    name: "neo-tree",
    description: "File system and other tree-like structures in a sidebar",
    category: PluginCategory::Explorer,
    event: None,
    cmd: Some("Neotree"),
    ft: None,
};

pub const PLUGIN_NVIM_TREE: Plugin = Plugin {
    repo: "nvim-tree/nvim-tree.lua",
    name: "nvim-tree",
    description: "A file explorer tree for Neovim written in Lua",
    category: PluginCategory::Explorer,
    event: None,
    cmd: Some("NvimTreeToggle"),
    ft: None,
};

pub const PLUGIN_OIL: Plugin = Plugin {
    repo: "stevearc/oil.nvim",
    name: "oil",
    description: "Edit your filesystem like a buffer",
    category: PluginCategory::Explorer,
    event: None,
    cmd: Some("Oil"),
    ft: None,
};

pub const PLUGIN_LUALINE: Plugin = Plugin {
    repo: "nvim-lualine/lualine.nvim",
    name: "lualine",
    description: "A blazing fast and easy to configure Neovim statusline",
    category: PluginCategory::Ui,
    event: Some("VimEnter"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_WHICH_KEY: Plugin = Plugin {
    repo: "folke/which-key.nvim",
    name: "which-key",
    description: "Displays a popup with possible key bindings for the command you started",
    category: PluginCategory::Ui,
    event: Some("VeryLazy"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_NOICE: Plugin = Plugin {
    repo: "folke/noice.nvim",
    name: "noice",
    description: "Replaces the UI for messages, cmdline and the popupmenu",
    category: PluginCategory::Ui,
    event: Some("VeryLazy"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_TELESCOPE: Plugin = Plugin {
    repo: "nvim-telescope/telescope.nvim",
    name: "telescope",
    description: "Highly extensible fuzzy finder over lists",
    category: PluginCategory::Telescope,
    event: None,
    cmd: Some("Telescope"),
    ft: None,
};

pub const PLUGIN_TELESCOPE_FZF: Plugin = Plugin {
    repo: "nvim-telescope/telescope-fzf-native.nvim",
    name: "telescope-fzf-native",
    description: "FZF sorter for telescope written in C",
    category: PluginCategory::Telescope,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_AUTOPAIRS: Plugin = Plugin {
    repo: "windwp/nvim-autopairs",
    name: "nvim-autopairs",
    description: "Auto-closes brackets, quotes, and more",
    category: PluginCategory::Editor,
    event: Some("InsertEnter"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_TODO_COMMENTS: Plugin = Plugin {
    repo: "folke/todo-comments.nvim",
    name: "todo-comments",
    description: "Highlight, list and search TODO, FIXME, HACK comments",
    category: PluginCategory::Editor,
    event: Some("BufReadPost"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_COMMENT: Plugin = Plugin {
    repo: "numToStr/Comment.nvim",
    name: "Comment",
    description: "Smart and powerful comment plugin for Neovim",
    category: PluginCategory::Editor,
    event: Some("BufReadPost"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_GITSIGNS: Plugin = Plugin {
    repo: "lewis6991/gitsigns.nvim",
    name: "gitsigns",
    description: "Git integration: signs, blame, hunk navigation",
    category: PluginCategory::Git,
    event: Some("BufReadPre"),
    cmd: None,
    ft: None,
};

pub const PLUGIN_NEOGIT: Plugin = Plugin {
    repo: "NeogitOrg/neogit",
    name: "neogit",
    description: "A Magit-inspired git interface for Neovim",
    category: PluginCategory::Git,
    event: None,
    cmd: Some("Neogit"),
    ft: None,
};

pub const PLUGIN_PLENARY: Plugin = Plugin {
    repo: "nvim-lua/plenary.nvim",
    name: "plenary",
    description: "All the Lua functions you need (required by many plugins)",
    category: PluginCategory::Ui,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_NVIM_WEB_DEVICONS: Plugin = Plugin {
    repo: "nvim-tree/nvim-web-devicons",
    name: "nvim-web-devicons",
    description: "File type icons (requires a Nerd Font)",
    category: PluginCategory::Ui,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_NEODEV: Plugin = Plugin {
    repo: "folke/neodev.nvim",
    name: "neodev",
    description: "Neovim Lua development environment setup (LSP for Neovim APIs)",
    category: PluginCategory::Lsp,
    event: None,
    cmd: None,
    ft: None,
};

pub const PLUGIN_MUI: Plugin = Plugin {
    repo: "MunifTanjim/nui.nvim",
    name: "nui",
    description: "UI component library for Neovim (required by neo-tree)",
    category: PluginCategory::Ui,
    event: None,
    cmd: None,
    ft: None,
};
