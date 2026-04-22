use crate::wizard::state::{Extra, WizardState};

pub fn generate(state: &WizardState) -> String {
    let mut parts: Vec<&str> = Vec::new();

    if state.has_extra(Extra::WhichKey) {
        parts.push(WHICH_KEY);
    }
    if state.has_extra(Extra::Autopairs) {
        parts.push(AUTOPAIRS);
    }
    if state.has_extra(Extra::TodoComments) {
        parts.push(TODO_COMMENTS);
    }
    if state.has_extra(Extra::Comment) {
        parts.push(COMMENT);
    }

    if parts.is_empty() {
        return String::new();
    }

    format!(
        "-- =============================================================================\n\
         -- extras.lua — quality-of-life editor plugins\n\
         -- =============================================================================\n\n\
         return {{\n{}\n}}\n",
        parts.join("\n")
    )
}

const WHICH_KEY: &str = r#"  -- ── which-key.nvim ───────────────────────────────────────────────────────
  -- Displays a popup showing available keybindings when you pause mid-sequence.
  -- Every keymap with a `desc` field is automatically registered.
  {
    "folke/which-key.nvim",
    event = "VeryLazy",
    config = function()
      require("which-key").setup({
        plugins = {
          marks     = true,   -- show marks
          registers = true,   -- show registers
          spelling  = {
            enabled     = true,
            suggestions = 20,
          },
        },
        win = {
          border = "rounded",
        },
        -- Group prefixes shown in the popup
        spec = {
          { "<leader>f", group = "Find / Telescope" },
          { "<leader>g", group = "Git" },
          { "<leader>c", group = "Code / LSP" },
        },
      })
    end,
  },"#;

const AUTOPAIRS: &str = r#"  -- ── nvim-autopairs ──────────────────────────────────────────────────────
  -- Automatically closes brackets, quotes, and other pairs.
  -- Integrates with nvim-cmp so <CR> doesn't break pair completion.
  {
    "windwp/nvim-autopairs",
    event = "InsertEnter",
    config = function()
      local autopairs = require("nvim-autopairs")
      autopairs.setup({
        check_ts   = true,    -- use Treesitter to check context (smarter pairs)
        ts_config  = {
          lua  = { "string" },  -- don't add pairs inside Lua strings
          rust = { "string_content" },
        },
        disable_filetype = { "TelescopePrompt", "spectre_panel" },
      })

      -- Hook into nvim-cmp so <CR> confirms completion AND closes the pair
      local cmp_autopairs = require("nvim-autopairs.completion.cmp")
      local ok, cmp = pcall(require, "cmp")
      if ok then
        cmp.event:on("confirm_done", cmp_autopairs.on_confirm_done())
      end
    end,
  },"#;

const TODO_COMMENTS: &str = r#"  -- ── todo-comments.nvim ──────────────────────────────────────────────────
  -- Highlights TODO, FIXME, HACK, WARN, NOTE, PERF, and TEST comments.
  -- Use :TodoTelescope to list all todos in the project.
  {
    "folke/todo-comments.nvim",
    event        = { "BufReadPost" },
    dependencies = { "nvim-lua/plenary.nvim" },
    config = function()
      require("todo-comments").setup({
        signs      = true,  -- show icons in the sign column
        sign_priority = 8,
        keywords = {
          FIX  = { icon = " ", color = "error",   alt = { "FIXME", "BUG", "FIXIT", "ISSUE" } },
          TODO = { icon = " ", color = "info" },
          HACK = { icon = " ", color = "warning", alt = { "XXX" } },
          WARN = { icon = " ", color = "warning", alt = { "WARNING" } },
          PERF = { icon = " ", color = "default", alt = { "OPTIM", "PERFORMANCE", "OPTIMIZE" } },
          NOTE = { icon = " ", color = "hint",    alt = { "INFO" } },
          TEST = { icon = "⏲ ", color = "test",  alt = { "TESTING", "PASSED", "FAILED" } },
        },
        highlight = {
          before         = "",      -- don't highlight text before the keyword
          keyword        = "wide",  -- highlight the keyword and its surroundings
          after          = "fg",    -- highlight text after the keyword
          pattern        = [[.*<(KEYWORDS)\s*:]],
          comments_only  = true,
        },
        search = {
          command = "rg",
          args    = { "--color=never", "--no-heading", "--with-filename", "--line-number", "--column" },
          pattern = [[\b(KEYWORDS):]],
        },
      })
    end,
  },"#;

const COMMENT: &str = r#"  -- ── Comment.nvim ───────────────────────────────────────────────────────
  -- Smart commenting for any filetype.
  -- gcc  — toggle line comment
  -- gbc  — toggle block comment
  -- gc   — operator-pending (gc5j to comment 5 lines down)
  {
    "numToStr/Comment.nvim",
    event = { "BufReadPost" },
    config = function()
      require("Comment").setup({
        -- Integration with Treesitter for context-aware commenting
        -- (e.g., HTML inside JSX uses the correct comment style)
        pre_hook = function(ctx)
          local ok, ts_comment = pcall(require, "ts_context_commentstring.integrations.comment_nvim")
          if ok then
            return ts_comment.create_pre_hook()(ctx)
          end
        end,
      })
    end,
  },"#;
