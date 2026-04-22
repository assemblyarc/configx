use strum::IntoEnumIterator;

use crate::events::AppEvent;
use crate::wizard::state::{Explorer, Extra, Feature, KeybindingStyle, Language, WizardState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Welcome,
    Languages,
    Features,
    Colorscheme,
    Explorer,
    Keybindings,
    Extras,
    Confirm,
}

impl Step {
    pub const ALL: &'static [Step] = &[
        Step::Welcome,
        Step::Languages,
        Step::Features,
        Step::Colorscheme,
        Step::Explorer,
        Step::Keybindings,
        Step::Extras,
        Step::Confirm,
    ];

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|s| *s == self).unwrap_or(0)
    }

    pub fn next(self) -> Option<Step> {
        let i = self.index() + 1;
        Self::ALL.get(i).copied()
    }

    pub fn prev(self) -> Option<Step> {
        let i = self.index();
        if i == 0 {
            None
        } else {
            Self::ALL.get(i - 1).copied()
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Step::Welcome => "Welcome to configx",
            Step::Languages => "Step 1 — Languages",
            Step::Features => "Step 2 — Editor Features",
            Step::Colorscheme => "Step 3 — Colorscheme",
            Step::Explorer => "Step 4 — File Explorer",
            Step::Keybindings => "Step 5 — Keybinding Style",
            Step::Extras => "Step 6 — Extra Plugins",
            Step::Confirm => "Step 7 — Confirm & Generate",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Step::Welcome => "Press Enter or → to start",
            Step::Languages | Step::Features | Step::Extras => {
                "↑↓ navigate  ·  Space toggle  ·  Enter next  ·  ← back  ·  q quit"
            }
            Step::Colorscheme | Step::Explorer | Step::Keybindings => {
                "↑↓ navigate  ·  Enter select  ·  ← back  ·  q quit"
            }
            Step::Confirm => "Press g to generate  ·  ← back  ·  q quit",
        }
    }
}

#[derive(Debug, Default)]
pub struct StepCursor {
    pub index: usize,
}

impl StepCursor {
    pub fn move_up(&mut self, len: usize) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = len.saturating_sub(1);
        }
    }

    pub fn move_down(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        self.index = (self.index + 1) % len;
    }
}

pub fn handle_step_event(
    step: Step,
    event: &AppEvent,
    cursor: &mut StepCursor,
    state: &mut WizardState,
) -> StepAction {
    match step {
        Step::Welcome => match event {
            AppEvent::Next => StepAction::Next,
            AppEvent::Quit => StepAction::Quit,
            _ => StepAction::None,
        },

        Step::Languages => {
            let langs: Vec<Language> = Language::iter().collect();
            handle_multiselect(event, cursor, &langs, &mut state.languages)
        }

        Step::Features => {
            let feats: Vec<Feature> = Feature::iter().collect();
            handle_multiselect(event, cursor, &feats, &mut state.features)
        }

        Step::Colorscheme => {
            let len = crate::registry::colorschemes::COLORSCHEMES.len();
            match event {
                AppEvent::Up => {
                    cursor.move_up(len);
                    StepAction::None
                }
                AppEvent::Down => {
                    cursor.move_down(len);
                    StepAction::None
                }
                AppEvent::Next | AppEvent::Toggle => {
                    state.colorscheme_index = cursor.index;
                    StepAction::Next
                }
                AppEvent::Back => StepAction::Back,
                AppEvent::Quit => StepAction::Quit,
                _ => StepAction::None,
            }
        }

        Step::Explorer => {
            let explorers: Vec<Explorer> = Explorer::iter().collect();
            handle_radio(event, cursor, &explorers, &mut state.explorer)
        }

        Step::Keybindings => {
            let styles: Vec<KeybindingStyle> = KeybindingStyle::iter().collect();
            handle_radio(event, cursor, &styles, &mut state.keybinding_style)
        }

        Step::Extras => {
            let extras: Vec<Extra> = Extra::iter().collect();
            handle_multiselect(event, cursor, &extras, &mut state.extras)
        }

        Step::Confirm => match event {
            AppEvent::Generate | AppEvent::Next => StepAction::Generate,
            AppEvent::Back => StepAction::Back,
            AppEvent::Quit => StepAction::Quit,
            _ => StepAction::None,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepAction {
    None,
    Next,
    Back,
    Generate,
    Quit,
}

fn handle_multiselect<T: PartialEq + Copy>(
    event: &AppEvent,
    cursor: &mut StepCursor,
    items: &[T],
    selected: &mut Vec<T>,
) -> StepAction {
    match event {
        AppEvent::Up => {
            cursor.move_up(items.len());
            StepAction::None
        }
        AppEvent::Down => {
            cursor.move_down(items.len());
            StepAction::None
        }
        AppEvent::Toggle => {
            if let Some(&item) = items.get(cursor.index) {
                if let Some(pos) = selected.iter().position(|x| *x == item) {
                    selected.remove(pos);
                } else {
                    selected.push(item);
                }
            }
            StepAction::None
        }
        AppEvent::Next => StepAction::Next,
        AppEvent::Back => StepAction::Back,
        AppEvent::Quit => StepAction::Quit,
        _ => StepAction::None,
    }
}

fn handle_radio<T: PartialEq + Copy>(
    event: &AppEvent,
    cursor: &mut StepCursor,
    items: &[T],
    selected: &mut T,
) -> StepAction {
    match event {
        AppEvent::Up => {
            cursor.move_up(items.len());
            StepAction::None
        }
        AppEvent::Down => {
            cursor.move_down(items.len());
            StepAction::None
        }
        AppEvent::Next | AppEvent::Toggle => {
            if let Some(&item) = items.get(cursor.index) {
                *selected = item;
            }
            StepAction::Next
        }
        AppEvent::Back => StepAction::Back,
        AppEvent::Quit => StepAction::Quit,
        _ => StepAction::None,
    }
}
