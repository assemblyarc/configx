use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    Go,
    #[strum(to_string = "C / C++")]
    C,
    Lua,
    Java,
    Zig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum Feature {
    #[strum(to_string = "LSP (language servers via mason.nvim)")]
    Lsp,
    #[strum(to_string = "Completion (nvim-cmp)")]
    Completion,
    #[strum(to_string = "Snippets (LuaSnip + friendly-snippets)")]
    Snippets,
    #[strum(to_string = "Treesitter (syntax / text objects)")]
    Treesitter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Default)]
pub enum Explorer {
    #[default]
    #[strum(to_string = "neo-tree  — sidebar tree with icons")]
    NeoTree,
    #[strum(to_string = "nvim-tree — classic file tree sidebar")]
    NvimTree,
    #[strum(to_string = "oil.nvim  — edit filesystem like a buffer")]
    Oil,
    #[strum(to_string = "None      — no file explorer")]
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Default)]
pub enum KeybindingStyle {
    #[default]
    #[strum(to_string = "Vim Purist   — hjkl, minimal leader bindings")]
    Purist,
    #[strum(to_string = "VSCode-Like  — Ctrl+P, Ctrl+F, Ctrl+S, familiar shortcuts")]
    VsCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum Extra {
    #[strum(to_string = "Telescope      — fuzzy finder")]
    Telescope,
    #[strum(to_string = "which-key      — keybinding popup hints")]
    WhichKey,
    #[strum(to_string = "gitsigns       — git diff in the gutter")]
    Gitsigns,
    #[strum(to_string = "Neogit         — Magit-style git UI")]
    Neogit,
    #[strum(to_string = "lualine        — statusline")]
    Lualine,
    #[strum(to_string = "autopairs      — auto-close brackets & quotes")]
    Autopairs,
    #[strum(to_string = "todo-comments  — highlight TODO / FIXME / HACK")]
    TodoComments,
    #[strum(to_string = "Comment.nvim   — smart line/block commenting")]
    Comment,
}

#[derive(Debug, Default, Clone)]
pub struct WizardState {
    pub languages: Vec<Language>,
    pub features: Vec<Feature>,
    pub colorscheme_index: usize,
    pub explorer: Explorer,
    pub keybinding_style: KeybindingStyle,
    pub extras: Vec<Extra>,
}

impl WizardState {
    pub fn has_feature(&self, f: Feature) -> bool {
        self.features.contains(&f)
    }

    pub fn has_extra(&self, e: Extra) -> bool {
        self.extras.contains(&e)
    }
}
