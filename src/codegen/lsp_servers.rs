use crate::wizard::state::Language;

pub struct LspEntry {
    pub mason_pkg: &'static str,
    pub lspconfig_name: &'static str,
    pub note: &'static str,
}

pub fn lsp_entries_for(lang: Language) -> &'static [LspEntry] {
    match lang {
        Language::Rust => &[LspEntry {
            mason_pkg: "rust-analyzer",
            lspconfig_name: "rust_analyzer",
            note: "Rust — full IDE experience via rust-analyzer",
        }],
        Language::Python => &[
            LspEntry {
                mason_pkg: "pyright",
                lspconfig_name: "pyright",
                note: "Python — Pyright for static type checking",
            },
            LspEntry {
                mason_pkg: "ruff",
                lspconfig_name: "ruff",
                note: "Python — Ruff for linting + formatting",
            },
        ],
        Language::TypeScript => &[LspEntry {
            mason_pkg: "typescript-language-server",
            lspconfig_name: "ts_ls",
            note: "TypeScript / JavaScript — tsserver",
        }],
        Language::Go => &[LspEntry {
            mason_pkg: "gopls",
            lspconfig_name: "gopls",
            note: "Go — gopls official language server",
        }],
        Language::C => &[LspEntry {
            mason_pkg: "clangd",
            lspconfig_name: "clangd",
            note: "C / C++ — clangd with fast indexing",
        }],
        Language::Lua => &[LspEntry {
            mason_pkg: "lua-language-server",
            lspconfig_name: "lua_ls",
            note: "Lua — lua-language-server with Neovim API support",
        }],
        Language::Java => &[LspEntry {
            mason_pkg: "jdtls",
            lspconfig_name: "jdtls",
            note: "Java — Eclipse JDT Language Server",
        }],
        Language::Zig => &[LspEntry {
            mason_pkg: "zls",
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
