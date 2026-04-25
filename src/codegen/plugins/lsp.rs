use crate::codegen::lsp_servers::{formatters_for, lsp_entries_for};
use crate::wizard::state::WizardState;

pub fn generate(state: &WizardState) -> String {
    let mut mason_pkgs: Vec<String> = Vec::new();
    let mut server_setups: Vec<String> = Vec::new();
    let mut formatter_map: Vec<String> = Vec::new();

    for &lang in &state.languages {
        for entry in lsp_entries_for(lang) {
            let pkg = format!("    \"{}\", -- {}", entry.lspconfig_name, entry.note);
            if !mason_pkgs.contains(&pkg) {
                mason_pkgs.push(pkg);
            }

            let setup = format!(
                "    lspconfig.{}.setup({{ capabilities = capabilities }})",
                entry.lspconfig_name
            );
            if !server_setups.contains(&setup) {
                server_setups.push(setup);
            }
        }

        let fmts = formatters_for(lang);
        if !fmts.is_empty() {
            let fmt_list = fmts
                .iter()
                .map(|f| format!("\"{}\"", f))
                .collect::<Vec<_>>()
                .join(", ");

            match lang {
                crate::wizard::state::Language::TypeScript => {
                    // TypeScript and JavaScript share the same formatter
                    formatter_map.push(format!("      [\"typescript\"] = {{ {fmt_list} }},"));
                    formatter_map.push(format!("      [\"javascript\"] = {{ {fmt_list} }},"));
                }
                _ => {
                    let k = lang
                        .to_string()
                        .to_lowercase()
                        .replace(" / ", "_")
                        .replace(" ", "_");
                    formatter_map.push(format!("      [\"{k}\"] = {{ {fmt_list} }},"));
                }
            }
        }
    }

    let lua_ls_pkg = "    \"lua-language-server\", -- Lua — for editing Neovim configs";
    if !mason_pkgs.iter().any(|p| p.contains("lua-language-server")) {
        mason_pkgs.push(lua_ls_pkg.to_string());
        server_setups.push(
            "    lspconfig.lua_ls.setup({\n      capabilities = capabilities,\n      settings = { Lua = { diagnostics = { globals = { \"vim\" } } } },\n    })".to_string(),
        );
    }

    let mason_list = mason_pkgs.join("\n");
    let setup_list = server_setups.join("\n");
    let fmt_list = formatter_map.join("\n");

    format!(
        r#"-- =============================================================================
-- lsp.lua — Language Server Protocol configuration
--
-- Stack:
--   mason.nvim         — installs LSP servers, formatters, and linters
--   mason-lspconfig    — bridges mason with nvim-lspconfig
--   nvim-lspconfig     — configures each language server
--   conform.nvim       — runs formatters on save
--   neodev.nvim        — Neovim Lua API completions (for editing init.lua)
-- =============================================================================

return {{{{
  -- ── mason.nvim ───────────────────────────────────────────────────────────
  -- Package manager for LSP servers, DAPs, linters, and formatters.
  -- Run :Mason to open the UI.
  {{
    "williamboman/mason.nvim",
    cmd    = "Mason",
    build  = ":MasonUpdate",
    config = function()
      require("mason").setup({{
        ui = {{ border = "rounded" }},
      }})
    end,
  }},

  -- ── mason-lspconfig ──────────────────────────────────────────────────────
  -- Automatically installs LSP servers listed below and hooks them into
  -- nvim-lspconfig.
  {{
    "williamboman/mason-lspconfig.nvim",
    dependencies = {{ "williamboman/mason.nvim" }},
    config = function()
      require("mason-lspconfig").setup({{
        ensure_installed = {{
{mason_list}
        }},
        automatic_installation = true,
      }})
    end,
  }},

  -- ── neodev ───────────────────────────────────────────────────────────────
  -- Must be set up BEFORE lspconfig so it can inject Neovim API types.
  {{
    "folke/neodev.nvim",
    opts = {{}},
  }},

  -- ── nvim-lspconfig ───────────────────────────────────────────────────────
  -- Configures every installed language server with sensible defaults and
  -- merges nvim-cmp's extended capabilities.
  {{
    "neovim/nvim-lspconfig",
    event = {{ "BufReadPre", "BufNewFile" }},
    dependencies = {{
      "williamboman/mason-lspconfig.nvim",
      "hrsh7th/cmp-nvim-lsp",
      "folke/neodev.nvim",
    }},
    config = function()
      local lspconfig = require("lspconfig")

      -- Extend LSP capabilities with nvim-cmp completion support.
      -- This tells the server what completion features the client supports.
      local capabilities = require("cmp_nvim_lsp").default_capabilities()

      -- ── Diagnostic appearance ──────────────────────────────────────────
      vim.diagnostic.config({{
        virtual_text  = true,
        signs         = true,
        underline     = true,
        update_in_insert = false,
        severity_sort = true,
        float = {{
          border = "rounded",
          source = "always",
        }},
      }})

      -- ── LSP keymaps (only active when an LSP attaches to a buffer) ─────
      vim.api.nvim_create_autocmd("LspAttach", {{
        group = vim.api.nvim_create_augroup("LspKeymaps", {{ clear = true }}),
        callback = function(event)
          local map = function(keys, func, desc)
            vim.keymap.set("n", keys, func, {{ buffer = event.buf, desc = "LSP: " .. desc }})
          end

          map("gd",        vim.lsp.buf.definition,       "Go to definition")
          map("gD",        vim.lsp.buf.declaration,      "Go to declaration")
          map("gr",        vim.lsp.buf.references,       "Find references")
          map("gi",        vim.lsp.buf.implementation,   "Go to implementation")
          map("K",         vim.lsp.buf.hover,            "Hover documentation")
          map("<leader>rn", vim.lsp.buf.rename,          "Rename symbol")
          map("<leader>ca", vim.lsp.buf.code_action,     "Code action")
          map("[d",        vim.diagnostic.goto_prev,     "Previous diagnostic")
          map("]d",        vim.diagnostic.goto_next,     "Next diagnostic")
          map("<leader>d", vim.diagnostic.open_float,    "Show diagnostics")
        end,
      }})

      -- ── Server setup ──────────────────────────────────────────────────
{setup_list}
    end,
  }},

  -- ── conform.nvim ─────────────────────────────────────────────────────────
  -- Lightweight formatter that runs the right tool per filetype.
  {{
    "stevearc/conform.nvim",
    event = "BufWritePre",
    config = function()
      require("conform").setup({{
        -- Map filetypes to formatters
        formatters_by_ft = {{
{fmt_list}
        }},
        -- Format on save
        format_on_save = {{
          timeout_ms = 500,
          lsp_fallback = true,   -- fall back to LSP formatting if no formatter is configured
        }},
      }})
    end,
  }},
}}}}
"#,
        mason_list = mason_list,
        setup_list = setup_list,
        fmt_list = fmt_list,
    )
}
