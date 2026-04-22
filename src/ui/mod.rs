pub mod widgets;

use ratatui::{
    Frame,
    style::{Modifier, Style},
    text::{Line, Span},
};
use strum::IntoEnumIterator;

use crate::registry::colorschemes::COLORSCHEMES;
use crate::wizard::{
    state::{Explorer, Extra, Feature, KeybindingStyle, Language, WizardState},
    steps::Step,
};
use widgets::*;

pub fn render_step(
    f: &mut Frame,
    step: Step,
    cursor: usize,
    state: &WizardState,
    status_message: &Option<String>,
) {
    let total = Step::ALL.len();
    let step_info = if step == Step::Welcome {
        String::new()
    } else {
        format!("[{}/{}]", step.index(), total - 1)
    };

    let content = render_chrome(f, step.title(), step.hint(), &step_info);

    match step {
        Step::Welcome => render_welcome(f, content),
        Step::Languages => render_languages(f, content, cursor, state),
        Step::Features => render_features(f, content, cursor, state),
        Step::Colorscheme => render_colorscheme(f, content, cursor, state),
        Step::Explorer => render_explorer(f, content, cursor, state),
        Step::Keybindings => render_keybindings(f, content, cursor, state),
        Step::Extras => render_extras(f, content, cursor, state),
        Step::Confirm => render_confirm(f, content, state, status_message),
    }
}

fn render_welcome(f: &mut Frame, area: ratatui::layout::Rect) {
    let lines = vec![
        Line::raw(""),
        Line::from(vec![Span::styled(
            "  ⚡ Welcome to configx",
            Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD),
        )]),
        Line::raw(""),
        Line::from(vec![Span::styled(
            "  Your offline, single-binary Neovim config wizard.",
            Style::default().fg(CLR_TEXT),
        )]),
        Line::raw(""),
        Line::from(vec![Span::styled(
            "  In a few steps you'll get a complete, modular, well-commented",
            Style::default().fg(CLR_SUBTEXT),
        )]),
        Line::from(vec![Span::styled(
            "  ~/.config/nvim/ directory — ready to drop in and use.",
            Style::default().fg(CLR_SUBTEXT),
        )]),
        Line::raw(""),
        Line::from(vec![Span::styled(
            "  What you'll configure:",
            Style::default().fg(CLR_YELLOW),
        )]),
        Line::from(vec![Span::styled(
            "    • Programming languages  →  auto-selects LSP servers",
            Style::default().fg(CLR_SUBTEXT),
        )]),
        Line::from(vec![Span::styled(
            "    • Editor features        →  LSP, completion, snippets, treesitter",
            Style::default().fg(CLR_SUBTEXT),
        )]),
        Line::from(vec![Span::styled(
            "    • Colorscheme            →  live ANSI preview",
            Style::default().fg(CLR_SUBTEXT),
        )]),
        Line::from(vec![Span::styled(
            "    • File explorer          →  neo-tree, nvim-tree, oil.nvim",
            Style::default().fg(CLR_SUBTEXT),
        )]),
        Line::from(vec![Span::styled(
            "    • Keybinding style       →  Vim Purist or VSCode-Like",
            Style::default().fg(CLR_SUBTEXT),
        )]),
        Line::from(vec![Span::styled(
            "    • Extra plugins          →  Telescope, which-key, git, and more",
            Style::default().fg(CLR_SUBTEXT),
        )]),
        Line::raw(""),
        Line::from(vec![Span::styled(
            "  Press Enter to begin →",
            Style::default().fg(CLR_GREEN).add_modifier(Modifier::BOLD),
        )]),
    ];

    render_centred_text(f, area, lines);
}

fn render_languages(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    cursor: usize,
    state: &WizardState,
) {
    let langs: Vec<Language> = Language::iter().collect();
    let items: Vec<String> = langs.iter().map(|l| l.to_string()).collect();
    let selected: Vec<usize> = langs
        .iter()
        .enumerate()
        .filter(|(_, l)| state.languages.contains(l))
        .map(|(i, _)| i)
        .collect();

    render_multiselect(
        f,
        area,
        "Select languages you write (Space to toggle, multiple allowed)",
        &items,
        &selected,
        cursor,
    );
}

fn render_features(f: &mut Frame, area: ratatui::layout::Rect, cursor: usize, state: &WizardState) {
    let feats: Vec<Feature> = Feature::iter().collect();
    let items: Vec<String> = feats.iter().map(|l| l.to_string()).collect();
    let selected: Vec<usize> = feats
        .iter()
        .enumerate()
        .filter(|(_, f)| state.features.contains(f))
        .map(|(i, _)| i)
        .collect();

    render_multiselect(f, area, "Select editor features", &items, &selected, cursor);
}

fn render_colorscheme(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    cursor: usize,
    state: &WizardState,
) {
    render_colorscheme_picker(f, area, COLORSCHEMES, state.colorscheme_index, cursor);
}

fn render_explorer(f: &mut Frame, area: ratatui::layout::Rect, cursor: usize, state: &WizardState) {
    let explorers: Vec<Explorer> = Explorer::iter().collect();
    let selected_index = explorers
        .iter()
        .position(|e| *e == state.explorer)
        .unwrap_or(0);

    render_radio(
        f,
        area,
        "Choose a file explorer",
        &explorers,
        selected_index,
        cursor,
    );
}

fn render_keybindings(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    cursor: usize,
    state: &WizardState,
) {
    let styles: Vec<KeybindingStyle> = KeybindingStyle::iter().collect();
    let selected_index = styles
        .iter()
        .position(|s| *s == state.keybinding_style)
        .unwrap_or(0);

    render_radio(
        f,
        area,
        "Choose a keybinding style",
        &styles,
        selected_index,
        cursor,
    );
}

fn render_extras(f: &mut Frame, area: ratatui::layout::Rect, cursor: usize, state: &WizardState) {
    let extras: Vec<Extra> = Extra::iter().collect();
    let items: Vec<String> = extras.iter().map(|e| e.to_string()).collect();
    let selected: Vec<usize> = extras
        .iter()
        .enumerate()
        .filter(|(_, e)| state.extras.contains(e))
        .map(|(i, _)| i)
        .collect();

    render_multiselect(
        f,
        area,
        "Select extra plugins (optional)",
        &items,
        &selected,
        cursor,
    );
}

fn render_confirm(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    state: &WizardState,
    status_message: &Option<String>,
) {
    use crate::registry::colorschemes::COLORSCHEMES;

    let scheme_name = COLORSCHEMES
        .get(state.colorscheme_index)
        .map(|s| s.name)
        .unwrap_or("catppuccin");

    let langs = if state.languages.is_empty() {
        "None selected".to_string()
    } else {
        state
            .languages
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };

    let features = if state.features.is_empty() {
        "None selected".to_string()
    } else {
        state
            .features
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };

    let extras = if state.extras.is_empty() {
        "None selected".to_string()
    } else {
        state
            .extras
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };

    let mut rows: Vec<(&str, String)> = vec![
        ("Languages", langs),
        ("Features", features),
        ("Colorscheme", scheme_name.to_string()),
        ("File Explorer", state.explorer.to_string()),
        ("Keybinding Style", state.keybinding_style.to_string()),
        ("Extra Plugins", extras),
        ("", "".to_string()),
        ("Output directory", "./nvim-output/".to_string()),
    ];

    if let Some(msg) = status_message {
        rows.push(("", "".to_string()));
        rows.push(("Status", msg.clone()));
    }

    render_summary(f, area, rows);
}

pub fn render_done(f: &mut Frame, output_path: &str) {
    let area = f.area();

    let lines = vec![
        Line::raw(""),
        Line::from(vec![Span::styled(
            "  ✓ Config generated successfully!",
            Style::default().fg(CLR_GREEN).add_modifier(Modifier::BOLD),
        )]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("  Output: ", Style::default().fg(CLR_MUTED)),
            Span::styled(output_path, Style::default().fg(CLR_ACCENT)),
        ]),
        Line::raw(""),
        Line::from(vec![Span::styled(
            "  To use this config:",
            Style::default().fg(CLR_YELLOW),
        )]),
        Line::from(vec![Span::styled(
            "    cp -r ./nvim-output ~/.config/nvim",
            Style::default().fg(CLR_SUBTEXT),
        )]),
        Line::raw(""),
        Line::from(vec![Span::styled(
            "  Then open Neovim — lazy.nvim will install all plugins on first launch.",
            Style::default().fg(CLR_SUBTEXT),
        )]),
        Line::raw(""),
        Line::from(vec![Span::styled(
            "  Press q to exit.",
            Style::default().fg(CLR_MUTED),
        )]),
    ];

    render_centred_text(f, area, lines);
}
