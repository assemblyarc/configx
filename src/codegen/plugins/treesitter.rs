use crate::codegen::lsp_servers::treesitter_parsers_for;
use crate::wizard::state::WizardState;

pub fn generate(state: &WizardState) -> String {
    let mut parsers: Vec<&str> = vec![
        "lua",
        "vim",
        "vimdoc",
        "query",
        "markdown",
        "markdown_inline",
        "bash",
        "json",
        "yaml",
        "toml",
    ];

    for lang in &state.languages {
        for parser in treesitter_parsers_for(*lang) {
            if !parsers.contains(parser) {
                parsers.push(parser);
            }
        }
    }

    let parser_list = parsers
        .iter()
        .map(|p| format!("    \"{p}\","))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"-- =============================================================================
-- treesitter.lua — syntax highlighting, indentation, and text objects
--
-- nvim-treesitter provides accurate, fast syntax highlighting using
-- Tree-sitter parsers instead of regex-based Vim syntax files.
-- =============================================================================

return {{{{
  -- Core Treesitter plugin
  "nvim-treesitter/nvim-treesitter",
  -- Build step: compile parsers after installation/update
  build  = ":TSUpdate",
  event  = {{ "BufReadPost", "BufNewFile" }},
  dependencies = {{
    -- Syntax-aware text objects (select a function, class, parameter, etc.)
    "nvim-treesitter/nvim-treesitter-textobjects",
  }},
  config = function()
    require("nvim-treesitter.config").setup({{
      -- Parsers to install automatically
      ensure_installed = {{
{parser_list}
      }},

      -- Install parsers synchronously on first run
      sync_install = false,

      -- Automatically install missing parsers on buffer open
      auto_install = true,

      -- ── Highlighting ─────────────────────────────────────────────────
      highlight = {{
        enable = true,
        -- Disable Vim's built-in regex highlighting when Tree-sitter takes over
        additional_vim_regex_highlighting = false,
      }},

      -- ── Indentation ──────────────────────────────────────────────────
      indent = {{ enable = true }},

      -- ── Text objects ─────────────────────────────────────────────────
      -- Select / move / swap by semantic units (function, class, param, ...)
      textobjects = {{
        select = {{
          enable    = true,
          lookahead = true,   -- jump forward to the next text object
          keymaps = {{
            ["af"] = {{ query = "@function.outer", desc = "Select outer function" }},
            ["if"] = {{ query = "@function.inner", desc = "Select inner function" }},
            ["ac"] = {{ query = "@class.outer",    desc = "Select outer class" }},
            ["ic"] = {{ query = "@class.inner",    desc = "Select inner class" }},
            ["aa"] = {{ query = "@parameter.outer", desc = "Select outer parameter" }},
            ["ia"] = {{ query = "@parameter.inner", desc = "Select inner parameter" }},
          }},
        }},
        move = {{
          enable              = true,
          set_jumps           = true,
          goto_next_start     = {{
            ["]f"] = {{ query = "@function.outer", desc = "Next function start" }},
            ["]c"] = {{ query = "@class.outer",    desc = "Next class start" }},
          }},
          goto_previous_start = {{
            ["[f"] = {{ query = "@function.outer", desc = "Prev function start" }},
            ["[c"] = {{ query = "@class.outer",    desc = "Prev class start" }},
          }},
        }},
      }},
    }})
  end,
}}}}
"#,
        parser_list = parser_list,
    )
}
