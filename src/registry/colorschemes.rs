#[derive(Debug, Clone)]
pub struct Colorscheme {
    pub name: &'static str,
    pub repo: &'static str,
    pub setup_call: &'static str,
    pub description: &'static str,
    pub swatches: &'static [(u8, u8, u8)],
}

pub const COLORSCHEMES: &[Colorscheme] = &[
    Colorscheme {
        name: "Catppuccin Mocha",
        repo: "catppuccin/nvim",
        setup_call: r#"require("catppuccin").setup({ flavour = "mocha" })
vim.cmd.colorscheme("catppuccin")"#,
        description: "Soothing pastel theme — the most popular modern Neovim scheme",
        swatches: &[
            (30, 30, 46),    // base (bg)
            (205, 214, 244), // text (fg)
            (180, 190, 254), // lavender
            (137, 220, 235), // sapphire
            (166, 227, 161), // green
            (243, 139, 168), // red
            (250, 179, 135), // peach
        ],
    },
    Colorscheme {
        name: "Tokyo Night",
        repo: "folke/tokyonight.nvim",
        setup_call: r#"require("tokyonight").setup({ style = "night" })
vim.cmd.colorscheme("tokyonight")"#,
        description: "A clean dark theme inspired by Tokyo at night",
        swatches: &[
            (26, 27, 38),    // bg
            (192, 202, 245), // fg
            (122, 162, 247), // blue
            (187, 154, 247), // purple
            (125, 207, 255), // cyan
            (247, 118, 142), // red
            (224, 175, 104), // yellow
        ],
    },
    Colorscheme {
        name: "Gruvbox",
        repo: "ellisonleao/gruvbox.nvim",
        setup_call: r#"require("gruvbox").setup({ contrast = "hard" })
vim.cmd.colorscheme("gruvbox")"#,
        description: "Retro groove colour scheme with warm earth tones",
        swatches: &[
            (29, 32, 33),    // bg hard
            (235, 219, 178), // fg
            (184, 187, 38),  // green
            (215, 153, 33),  // yellow
            (214, 93, 14),   // orange
            (204, 36, 29),   // red
            (69, 133, 136),  // aqua
        ],
    },
    Colorscheme {
        name: "Rose Pine",
        repo: "rose-pine/neovim",
        setup_call: r#"require("rose-pine").setup({ variant = "main" })
vim.cmd.colorscheme("rose-pine")"#,
        description: "All natural pine, faux fur and a bit of soho vibes",
        swatches: &[
            (25, 23, 36),    // base
            (224, 222, 244), // text
            (196, 167, 231), // iris
            (49, 116, 143),  // pine
            (156, 207, 216), // foam
            (235, 111, 146), // love
            (246, 193, 119), // gold
        ],
    },
    Colorscheme {
        name: "Nord",
        repo: "shaunsingh/nord.nvim",
        setup_call: r#"vim.g.nord_contrast = true
require("nord").set()"#,
        description: "An arctic, north-bluish clean and elegant colour palette",
        swatches: &[
            (46, 52, 64),    // polar night 1
            (216, 222, 233), // snow storm 3
            (129, 161, 193), // frost 1
            (136, 192, 208), // frost 2
            (163, 190, 140), // aurora green
            (191, 97, 106),  // aurora red
            (235, 203, 139), // aurora yellow
        ],
    },
    Colorscheme {
        name: "OneDark",
        repo: "navarasu/onedark.nvim",
        setup_call: r#"require("onedark").setup({ style = "dark" })
require("onedark").load()"#,
        description: "Atom One Dark inspired theme with multiple style variants",
        swatches: &[
            (40, 44, 52),    // bg
            (171, 178, 191), // fg
            (97, 175, 239),  // blue
            (198, 120, 221), // purple
            (86, 182, 194),  // cyan
            (224, 108, 117), // red
            (229, 192, 123), // yellow
        ],
    },
];
