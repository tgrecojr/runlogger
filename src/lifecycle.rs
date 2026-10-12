//! Startup, the event loop and shutdown. Every terminal side effect is passed
//! in, so the order and guarantees can be tested without a real terminal.

use crate::app::App;
use crate::{input, ui};
use anyhow::Result;
use chrono::Local;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use rusqlite::Connection;
use std::io;
use std::panic;
use std::time::Duration;

/// Longest wait for input before the screen is redrawn anyway.
pub const IDLE_REDRAW: Duration = Duration::from_millis(100);

/// Where key presses come from: the real terminal, or a script in tests.
pub trait EventSource {
    /// The next event, or `None` if nothing arrived within `timeout`.
    fn next_event(&mut self, timeout: Duration) -> io::Result<Option<Event>>;
}

/// Events from the terminal via crossterm.
pub struct CrosstermEvents;

impl EventSource for CrosstermEvents {
    fn next_event(&mut self, timeout: Duration) -> io::Result<Option<Event>> {
        if event::poll(timeout)? {
            event::read().map(Some)
        } else {
            Ok(None)
        }
    }
}

/// Opens the database first, and takes over the terminal only if that
/// succeeded, so startup errors print to a normal terminal.
// @spec NAV-001
pub fn start<T>(
    open_db: impl FnOnce() -> Result<Connection>,
    enter_terminal: impl FnOnce() -> Result<T>,
) -> Result<(Connection, T)> {
    let conn = open_db()?;
    let terminal = enter_terminal()?;
    Ok((conn, terminal))
}

/// Runs `body`, then always runs `restore`; `body`'s error wins if both fail.
// @spec NAV-002
pub fn with_restore<S, T>(
    state: &mut S,
    body: impl FnOnce(&mut S) -> Result<T>,
    restore: impl FnOnce(&mut S) -> Result<()>,
) -> Result<T> {
    let result = body(state);
    let restored = restore(state);
    let value = result?;
    restored?;
    Ok(value)
}

/// Wraps the current panic hook so `restore` runs before the panic prints.
// @spec NAV-003
pub fn install_panic_hook(restore: impl Fn() + Send + Sync + 'static) {
    let original = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        restore();
        original(info);
    }));
}

/// Ticks the app clock, draws, then waits at most `IDLE_REDRAW` for an event,
/// until the app asks to quit.
// @spec NAV-004
pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    conn: &Connection,
    events: &mut impl EventSource,
) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    loop {
        app.tick(Local::now().date_naive());
        terminal.draw(|f| ui::render(f, app))?;

        if let Some(Event::Key(key)) = events.next_event(IDLE_REDRAW)? {
            input::handle_key_event(app, key, conn);
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

pub type CrosstermTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub fn enter_terminal() -> Result<CrosstermTerminal> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

pub fn restore_terminal(terminal: &mut CrosstermTerminal) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

/// Best-effort restore from inside the panic hook.
pub fn restore_after_panic() {
    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
}

#[cfg(test)]
mod tests;
