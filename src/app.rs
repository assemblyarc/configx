use std::{io, time::Duration};

use anyhow::{Context, Result};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    Args, codegen,
    events::{AppEvent, poll_event},
    ui,
    wizard::{
        state::WizardState,
        steps::{Step, StepAction, StepCursor, handle_step_event},
    },
};

#[derive(Debug)]
enum AppState {
    Wizard { step: Step, cursor: StepCursor },
    Done { output_path: String },
}

pub struct App {
    args: Args,
    state: AppState,
    wizard_state: WizardState,
    status_message: Option<String>,
}

impl App {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            state: AppState::Wizard {
                step: Step::Welcome,
                cursor: StepCursor::default(),
            },
            wizard_state: WizardState::default(),
            status_message: None,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;

            let Some(event) = poll_event(Duration::from_millis(16))? else {
                continue;
            };

            if event == AppEvent::Quit {
                return Ok(());
            }

            if self.handle_event(event)? {
                return Ok(());
            }
        }
    }

    fn draw(&self, f: &mut ratatui::Frame) {
        match &self.state {
            AppState::Wizard { step, cursor } => {
                ui::render_step(
                    f,
                    *step,
                    cursor.index,
                    &self.wizard_state,
                    &self.status_message,
                );
            }
            AppState::Done { output_path } => {
                ui::render_done(f, output_path);
            }
        }
    }

    fn handle_event(&mut self, event: AppEvent) -> Result<bool> {
        let (current_step, action) = match &mut self.state {
            AppState::Wizard { step, cursor } => {
                let current_step = *step;
                let action =
                    handle_step_event(current_step, &event, cursor, &mut self.wizard_state);
                (current_step, action)
            }
            AppState::Done { .. } => return Ok(true),
        };

        match action {
            StepAction::None => {}

            StepAction::Next => {
                if let Some(next) = current_step.next() {
                    let mut new_cursor = StepCursor::default();
                    self.sync_cursor_to_selection(next, &mut new_cursor);
                    self.state = AppState::Wizard {
                        step: next,
                        cursor: new_cursor,
                    };
                }
            }

            StepAction::Back => {
                if let Some(prev) = current_step.prev() {
                    let mut new_cursor = StepCursor::default();
                    self.sync_cursor_to_selection(prev, &mut new_cursor);
                    self.state = AppState::Wizard {
                        step: prev,
                        cursor: new_cursor,
                    };
                }
            }

            StepAction::Generate => {
                self.status_message = Some("Generating…".into());
                let output_path = self.generate()?;
                self.state = AppState::Done { output_path };
            }

            StepAction::Quit => return Ok(true),
        }

        Ok(false)
    }

    fn sync_cursor_to_selection(&self, step: Step, cursor: &mut StepCursor) {
        use strum::IntoEnumIterator;

        match step {
            Step::Explorer => {
                let explorers: Vec<_> = crate::wizard::state::Explorer::iter().collect();
                cursor.index = explorers
                    .iter()
                    .position(|e| *e == self.wizard_state.explorer)
                    .unwrap_or(0);
            }
            Step::Keybindings => {
                let styles: Vec<_> = crate::wizard::state::KeybindingStyle::iter().collect();
                cursor.index = styles
                    .iter()
                    .position(|s| *s == self.wizard_state.keybinding_style)
                    .unwrap_or(0);
            }
            Step::Colorscheme => {
                cursor.index = self.wizard_state.colorscheme_index;
            }
            _ => {}
        }
    }

    fn generate(&self) -> Result<String> {
        let output_dir = self
            .args
            .output
            .clone()
            .unwrap_or_else(|| std::path::PathBuf::from("./nvim-output"));

        codegen::generate(&self.wizard_state, &output_dir, self.args.dry_run)
            .with_context(|| "Code generation failed")?;

        Ok(output_dir.display().to_string())
    }
}
