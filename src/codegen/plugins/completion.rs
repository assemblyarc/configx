use crate::wizard::state::{Feature, WizardState};

pub fn generate(state: &WizardState) -> String {
    let has_snippets = state.has_feature(Feature::Snippets);
    let has_lsp = state.has_feature(Feature::Lsp);

    let snippet_dep = if has_snippets {
        r#"    { "L3MON4D3/LuaSnip", build = "make install_jsregexp" },
    "saadparwaiz1/cmp_luasnip",
    "rafamadriz/friendly-snippets","#
    } else {
        r#"    { "L3MON4D3/LuaSnip", build = "make install_jsregexp" },
    "saadparwaiz1/cmp_luasnip","#
    };

    let lsp_dep = if has_lsp {
        "    \"hrsh7th/cmp-nvim-lsp\","
    } else {
        ""
    };

    let lsp_source = if has_lsp {
        "      { name = \"nvim_lsp\" },   -- LSP completions"
    } else {
        ""
    };

    let snippet_source = "      { name = \"luasnip\" },    -- snippet completions";
    let snippet_config = if has_snippets {
        r#"
      -- Load friendly-snippets (a large community snippet collection)
      require("luasnip.loaders.from_vscode").lazy_load()"#
    } else {
        ""
    };

    format!(
        r#"-- =============================================================================
-- completion.lua — autocompletion engine
--
-- Stack:
--   nvim-cmp            — completion framework
--   LuaSnip             — snippet engine
--   friendly-snippets   — community snippet collection (VSCode format)
--   cmp sources         — LSP, snippets, buffer words, file paths
-- =============================================================================

return {{{{
  -- ── nvim-cmp ─────────────────────────────────────────────────────────────
  "hrsh7th/nvim-cmp",
  event        = "InsertEnter",
  dependencies = {{
{snippet_dep}
{lsp_dep}
    "hrsh7th/cmp-buffer",   -- words from the current buffer
    "hrsh7th/cmp-path",     -- file system paths
  }},
  config = function()
    local cmp     = require("cmp")
    local luasnip = require("luasnip"){snippet_config}

    cmp.setup({{
      -- ── Snippet engine ───────────────────────────────────────────────
      snippet = {{
        expand = function(args)
          luasnip.lsp_expand(args.body)
        end,
      }},

      -- ── Completion window appearance ─────────────────────────────────
      window = {{
        completion    = cmp.config.window.bordered(),
        documentation = cmp.config.window.bordered(),
      }},

      -- ── Key mappings ─────────────────────────────────────────────────
      mapping = cmp.mapping.preset.insert({{
        ["<C-n>"]   = cmp.mapping.select_next_item(),     -- next item
        ["<C-p>"]   = cmp.mapping.select_prev_item(),     -- previous item
        ["<C-b>"]   = cmp.mapping.scroll_docs(-4),        -- scroll doc up
        ["<C-f>"]   = cmp.mapping.scroll_docs(4),         -- scroll doc down
        ["<C-Space>"] = cmp.mapping.complete(),           -- trigger completion
        ["<C-e>"]   = cmp.mapping.abort(),                -- dismiss popup
        ["<CR>"]    = cmp.mapping.confirm({{ select = true }}), -- confirm selection
        -- Tab / S-Tab: cycle through items or jump snippet placeholders
        ["<Tab>"]   = cmp.mapping(function(fallback)
          if cmp.visible() then
            cmp.select_next_item()
          elseif luasnip.expand_or_jumpable() then
            luasnip.expand_or_jump()
          else
            fallback()
          end
        end, {{ "i", "s" }}),
        ["<S-Tab>"] = cmp.mapping(function(fallback)
          if cmp.visible() then
            cmp.select_prev_item()
          elseif luasnip.jumpable(-1) then
            luasnip.jump(-1)
          else
            fallback()
          end
        end, {{ "i", "s" }}),
      }}),

      -- ── Completion sources (ordered by priority) ──────────────────────
      sources = cmp.config.sources({{
{lsp_source}
{snippet_source}
      }}, {{
        {{ name = "buffer" }},   -- buffer words as fallback
        {{ name = "path" }},     -- file paths
      }}),

      -- ── Item formatting ───────────────────────────────────────────────
      formatting = {{
        format = function(entry, item)
          -- Show the source name next to each completion item
          local source_labels = {{
            nvim_lsp = "[LSP]",
            luasnip  = "[Snip]",
            buffer   = "[Buf]",
            path     = "[Path]",
          }}
          item.menu = source_labels[entry.source.name] or ""
          return item
        end,
      }},
    }})
  end,
}}}}
"#,
        snippet_dep = snippet_dep,
        lsp_dep = lsp_dep,
        lsp_source = lsp_source,
        snippet_source = snippet_source,
        snippet_config = snippet_config,
    )
}
