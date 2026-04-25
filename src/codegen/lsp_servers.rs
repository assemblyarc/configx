use crate::wizard::state::Language;

pub struct LspEntry {
    pub lspconfig_name: &'static str,
    pub note: &'static str,
}

pub fn lsp_entries_for(lang: Language) -> &'static [LspEntry] {
    match lang {
        Language::Rust => &[LspEntry {
            lspconfig_name: "rust_analyzer",
            note: "Rust — full IDE experience via rust-analyzer",
        }],
        Language::Python => &[
            LspEntry {
                lspconfig_name: "pyright",
                note: "Python — Pyright for static type checking",
            },
            LspEntry {
                lspconfig_name: "ruff",
                note: "Python — Ruff for linting + formatting",
            },
        ],
        Language::TypeScript => &[LspEntry {
            lspconfig_name: "ts_ls",
            note: "TypeScript / JavaScript — tsserver",
        }],
        Language::Go => &[LspEntry {
            lspconfig_name: "gopls",
            note: "Go — gopls official language server",
        }],
        Language::C => &[LspEntry {
            lspconfig_name: "clangd",
            note: "C / C++ — clangd with fast indexing",
        }],
        Language::Lua => &[LspEntry {
            lspconfig_name: "lua_ls",
            note: "Lua — lua-language-server with Neovim API support",
        }],
        Language::Java => &[LspEntry {
            lspconfig_name: "jdtls",
            note: "Java — Eclipse JDT Language Server",
        }],
        Language::Zig => &[LspEntry {
            lspconfig_name: "zls",
            note: "Zig — ZLS (Zig Language Server)",
        }],
    }
}

pub fn treesitter_parsers_for(lang: Language) -> &'static [&'static str] {
    match lang {
        Language::Rust => &["rust"],
        Language::Python => &["python"],
        Language::TypeScript => &["typescript", "tsx", "javascript"],
        Language::Go => &["go", "gomod", "gosum"],
        Language::C => &["c", "cpp"],
        Language::Lua => &["lua", "luadoc"],
        Language::Java => &["java"],
        Language::Zig => &["zig"],
    }
}

pub fn formatters_for(lang: Language) -> &'static [&'static str] {
    match lang {
        Language::Rust => &["rustfmt"],
        Language::Python => &["ruff_format"],
        Language::TypeScript => &["prettierd"],
        Language::Go => &["gofmt"],
        Language::C => &["clang-format"],
        Language::Lua => &["stylua"],
        Language::Java => &[],
        Language::Zig => &["zigfmt"],
    }
}
