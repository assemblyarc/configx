use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
    Up,
    Down,
    Toggle,
    Next,
    Back,
    Generate,
    Quit,
    Key(KeyEvent),
}

pub fn poll_event(timeout: Duration) -> Result<Option<AppEvent>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }

    match event::read()? {
        Event::Key(key) => Ok(Some(translate_key(key))),
        _ => Ok(None),
    }
}

fn translate_key(key: KeyEvent) -> AppEvent {
    if key.modifiers.contains(KeyModifiers::CONTROL)
        && matches!(key.code, KeyCode::Char('c') | KeyCode::Char('q'))
    {
        return AppEvent::Quit;
    }

    match key.code {
        KeyCode::Char('q') => AppEvent::Quit,
        KeyCode::Up | KeyCode::Char('k') => AppEvent::Up,
        KeyCode::Down | KeyCode::Char('j') => AppEvent::Down,
        KeyCode::Char(' ') => AppEvent::Toggle,
        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => AppEvent::Next,
        KeyCode::Left | KeyCode::Backspace | KeyCode::Char('h') => AppEvent::Back,
        KeyCode::Char('g') => AppEvent::Generate,
        _ => AppEvent::Key(key),
    }
}
