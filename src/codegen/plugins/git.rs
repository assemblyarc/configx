use crate::wizard::state::{Extra, WizardState};

pub fn generate(state: &WizardState) -> String {
    let has_gitsigns = state.has_extra(Extra::Gitsigns);
    let has_neogit = state.has_extra(Extra::Neogit);

    let mut specs: Vec<&str> = Vec::new();

    if has_gitsigns {
        specs.push(GITSIGNS);
    }
    if has_neogit {
        specs.push(NEOGIT);
    }

    if specs.is_empty() {
        return String::new();
    }

    format!(
        "-- =============================================================================\n\
         -- git.lua — Git integration\n\
         -- =============================================================================\n\n\
         return {{\n{}\n}}\n",
        specs.join("\n")
    )
}

const GITSIGNS: &str = r#"  -- ── gitsigns.nvim ──────────────────────────────────────────────────────────
  -- Shows git diff markers in the sign column and provides hunk operations.
  {
    "lewis6991/gitsigns.nvim",
    event = { "BufReadPre", "BufNewFile" },
    config = function()
      require("gitsigns").setup({
        signs = {
          add          = { text = "▎" },
          change       = { text = "▎" },
          delete       = { text = "" },
          topdelete    = { text = "" },
          changedelete = { text = "▎" },
          untracked    = { text = "▎" },
        },
        current_line_blame = false,  -- set to true to show inline blame
        current_line_blame_opts = {
          virt_text = true,
          virt_text_pos = "eol",
          delay = 1000,
        },
        -- Hunk preview in a floating window
        preview_config = {
          border = "rounded",
        },
      })
    end,
  },"#;

const NEOGIT: &str = r#"  -- ── neogit ──────────────────────────────────────────────────────────────────
  -- A Magit-inspired git interface.  Open with <leader>gg.
  {
    "NeogitOrg/neogit",
    cmd          = "Neogit",
    dependencies = {
      "nvim-lua/plenary.nvim",
      "sindrets/diffview.nvim",   -- beautiful side-by-side diffs
      "nvim-telescope/telescope.nvim",
    },
    config = function()
      require("neogit").setup({
        graph_style = "unicode",
        integrations = {
          telescope  = true,
          diffview   = true,
        },
        -- Open in a floating window for a modal git experience
        kind = "split",
      })
    end,
  },"#;
