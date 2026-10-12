//! Tests for startup order, terminal restore, the panic hook and the event loop.

use super::*;
use crate::test_support::{buffer_text, ch, memory_db};
use anyhow::anyhow;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::backend::TestBackend;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Plays back a script of events, recording every timeout it was asked to wait.
/// Once the script runs out it sends Ctrl+Q so the loop always ends.
struct Scripted {
    events: VecDeque<Option<Event>>,
    timeouts: Vec<Duration>,
}

impl Scripted {
    fn new(events: Vec<Option<Event>>) -> Self {
        Scripted {
            events: events.into(),
            timeouts: Vec::new(),
        }
    }
}

impl EventSource for Scripted {
    fn next_event(&mut self, timeout: Duration) -> io::Result<Option<Event>> {
        self.timeouts.push(timeout);
        Ok(self.events.pop_front().unwrap_or_else(|| {
            Some(Event::Key(KeyEvent::new(
                KeyCode::Char('q'),
                KeyModifiers::CONTROL,
            )))
        }))
    }
}

fn test_terminal() -> Terminal<TestBackend> {
    Terminal::new(TestBackend::new(100, 40)).unwrap()
}

// @spec NAV-001
#[test]
fn database_opens_before_the_terminal_is_taken_over() {
    let order = RefCell::new(Vec::new());
    start(
        || {
            order.borrow_mut().push("open database");
            Ok(memory_db())
        },
        || {
            order.borrow_mut().push("enter terminal");
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(*order.borrow(), ["open database", "enter terminal"]);
}

// @spec NAV-001
#[test]
fn terminal_is_never_touched_when_the_database_fails() {
    let entered = RefCell::new(false);
    let result = start(
        || Err(anyhow!("Failed to open database connection")),
        || {
            *entered.borrow_mut() = true;
            Ok(())
        },
    );
    assert!(result.is_err());
    assert!(!*entered.borrow());
}

// @spec NAV-001
#[test]
fn first_screen_is_quick_entry() {
    let conn = memory_db();
    let mut terminal = test_terminal();
    let mut app = App::new();
    run_app(&mut terminal, &mut app, &conn, &mut Scripted::new(vec![])).unwrap();
    assert!(buffer_text(terminal.backend().buffer()).contains("Quick Entry - Log Your Run"));
}

fn count_restore(restores: &mut i32) -> Result<()> {
    *restores += 1;
    Ok(())
}

// @spec NAV-002
#[test]
fn terminal_is_restored_after_a_normal_exit_and_after_an_error() {
    let mut restores = 0;
    let ok = with_restore(&mut restores, |_| Ok(7), count_restore);
    assert_eq!(ok.unwrap(), 7);
    assert_eq!(restores, 1);

    let failed: Result<()> = with_restore(
        &mut restores,
        |_| Err(anyhow!("draw failed")),
        count_restore,
    );
    assert_eq!(failed.unwrap_err().to_string(), "draw failed");
    assert_eq!(restores, 2);
}

// @spec NAV-002
#[test]
fn a_failed_restore_is_reported() {
    let result = with_restore(
        &mut (),
        |_| Ok(()),
        |_| Err(anyhow!("could not leave raw mode")),
    );
    assert_eq!(result.unwrap_err().to_string(), "could not leave raw mode");
}

// @spec NAV-003
#[test]
fn panic_restores_the_terminal_before_the_message_prints() {
    let order = Arc::new(Mutex::new(Vec::new()));
    let printed = Arc::clone(&order);
    panic::set_hook(Box::new(move |_| {
        printed.lock().unwrap().push("original hook prints")
    }));
    let restored = Arc::clone(&order);
    install_panic_hook(move || restored.lock().unwrap().push("restore terminal"));

    let caught = panic::catch_unwind(|| panic!("boom"));
    let _ = panic::take_hook(); // back to the default hook

    assert!(caught.is_err());
    assert_eq!(
        *order.lock().unwrap(),
        ["restore terminal", "original hook prints"]
    );
}

// @spec NAV-004
#[test]
fn loop_redraws_after_every_key_and_waits_at_most_100ms_while_idle() {
    let conn = memory_db();
    let mut terminal = test_terminal();
    let mut app = App::new();
    let mut events = Scripted::new(vec![None, None, Some(Event::Key(ch('5')))]);

    run_app(&mut terminal, &mut app, &conn, &mut events).unwrap();

    // One wait per frame: two idle frames, one after '5', one before Ctrl+Q.
    assert_eq!(events.timeouts.len(), 4);
    assert!(events
        .timeouts
        .iter()
        .all(|t| *t <= Duration::from_millis(100)));
    // The frame drawn after '5' shows it in the Distance field.
    assert!(buffer_text(terminal.backend().buffer()).contains("5_"));
}
