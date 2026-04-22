use crate::registry::colorschemes::COLORSCHEMES;
use crate::wizard::state::WizardState;

pub fn generate(state: &WizardState) -> String {
    let scheme = match COLORSCHEMES.get(state.colorscheme_index) {
        Some(s) => s,
        None => return String::new(),
    };

    format!(
        r#"-- =============================================================================
-- colorscheme.lua — colorscheme configuration
--
-- Plugin : {repo}
-- {description}
-- =============================================================================

return {{{{
  "{repo}",
  -- Give the colorscheme the highest priority so it loads before other UI plugins.
  priority = 1000,
  lazy     = false,
  config   = function()
    -- Plugin setup
    {setup_call}
  end,
}}}}
"#,
        repo = scheme.repo,
        description = scheme.description,
        setup_call = scheme.setup_call,
    )
}
