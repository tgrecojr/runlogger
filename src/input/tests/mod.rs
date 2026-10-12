use super::*;
use crate::test_support::{block_writes, ch, insert_raw, key, memory_db};

mod edits;
mod errors;
mod key_events;
mod refresh;
mod routing;

fn press(app: &mut App, conn: &Connection, keys: &[KeyEvent]) {
    for k in keys {
        handle_key_event(app, *k, conn);
    }
}

fn run_count(conn: &Connection) -> i64 {
    conn.query_row("SELECT COUNT(*) FROM runs", [], |r| r.get(0))
        .unwrap()
}

fn rl_error(app: &App) -> &str {
    app.run_list_state.error_message.as_deref().unwrap_or("")
}

fn qe(app: &App) -> &crate::app::QuickEntryState {
    &app.quick_entry_state
}

fn type_text(app: &mut App, conn: &Connection, text: &str) {
    for c in text.chars() {
        handle_key_event(app, ch(c), conn);
    }
}

fn three_runs(conn: &Connection) -> [i64; 3] {
    [
        insert_raw(conn, "2026-10-03", "07:00:00", 3.0),
        insert_raw(conn, "2026-10-02", "07:00:00", 2.0),
        insert_raw(conn, "2026-10-01", "07:00:00", 1.0),
    ]
}

fn selected_id(app: &App) -> Option<i64> {
    let state = &app.run_list_state;
    state.runs.get(state.selected_index).and_then(|r| r.id)
}

const ALL_SCREENS: [Screen; 4] = [
    Screen::QuickEntry,
    Screen::RunList,
    Screen::Analytics,
    Screen::Help,
];

const OTHER_SCREENS: [Screen; 3] = [Screen::RunList, Screen::Analytics, Screen::Help];

fn ctrl(c: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
}

fn app_on(screen: Screen) -> App {
    let mut app = App::new();
    app.switch_to_screen(screen);
    app
}
