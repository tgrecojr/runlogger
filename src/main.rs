mod app;
mod db;
mod logic;
mod models;
mod ui;

use anyhow::Result;
use app::{App, InputField, Screen};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use rusqlite::Connection;
use std::io;
use std::panic;
use std::time::Duration;

fn main() -> Result<()> {
    setup_panic_hook();

    let db_path = db::connection::get_db_path()?;
    let conn = db::connection::init_db(&db_path)?;

    let mut terminal = setup_terminal()?;
    let mut app = App::new();

    let result = run_app(&mut terminal, &mut app, &conn);

    restore_terminal(&mut terminal)?;

    result
}

fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal_on_panic();
        original_hook(panic_info);
    }));
}

fn restore_terminal_on_panic() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    conn: &Connection,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                handle_key_event(app, key, conn)?;
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent, conn: &Connection) -> Result<()> {
    // Ctrl+Q or Ctrl+C to quit from anywhere
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if matches!(key.code, KeyCode::Char('q') | KeyCode::Char('c')) {
            app.quit();
            return Ok(());
        }
    }

    // If waiting for navigation command after Escape
    if app.waiting_for_nav {
        app.waiting_for_nav = false;
        match key.code {
            KeyCode::Char('1') => {
                app.switch_to_screen(Screen::QuickEntry);
                return Ok(());
            }
            KeyCode::Char('2') => {
                load_runs(app, conn)?;
                app.switch_to_screen(Screen::RunList);
                return Ok(());
            }
            KeyCode::Char('3') => {
                load_analytics(app, conn)?;
                app.switch_to_screen(Screen::Analytics);
                return Ok(());
            }
            KeyCode::Esc => {
                // Second Escape press - clear Quick Entry fields if on that screen
                if app.screen == Screen::QuickEntry {
                    app.quick_entry_state.clear();
                }
                return Ok(());
            }
            _ => {
                // Any other key cancels navigation mode
                return Ok(());
            }
        }
    }

    // Escape enters navigation mode (instead of clearing fields)
    if key.code == KeyCode::Esc {
        app.waiting_for_nav = true;
        return Ok(());
    }

    // Regular 'q' to quit when not in Quick Entry
    if key.code == KeyCode::Char('q') && app.screen != Screen::QuickEntry {
        app.quit();
        return Ok(());
    }

    // Handle screen-specific input first for Quick Entry to allow typing numbers and letters
    if app.screen == Screen::QuickEntry {
        handle_quick_entry_input(app, key, conn)?;
        return Ok(());
    }

    // Global navigation keys (only when not in Quick Entry)
    match key.code {
        KeyCode::Char('1') => app.switch_to_screen(Screen::QuickEntry),
        KeyCode::Char('2') => {
            load_runs(app, conn)?;
            app.switch_to_screen(Screen::RunList);
        }
        KeyCode::Char('3') => {
            load_analytics(app, conn)?;
            app.switch_to_screen(Screen::Analytics);
        }
        KeyCode::Char('h') | KeyCode::Char('?') => app.switch_to_screen(Screen::Help),
        _ => {}
    }

    // Handle other screens
    match app.screen {
        Screen::QuickEntry => {}, // Already handled above
        Screen::RunList => handle_run_list_input(app, key, conn)?,
        Screen::Analytics => {}
        Screen::Help => {}
    }

    Ok(())
}

fn handle_quick_entry_input(app: &mut App, key: KeyEvent, conn: &Connection) -> Result<()> {
    let state = &mut app.quick_entry_state;

    match key.code {
        KeyCode::Tab => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                state.prev_field();
            } else {
                state.next_field();
            }
        }
        KeyCode::BackTab => {
            state.prev_field();
        }
        KeyCode::Enter => {
            state.error_message = None;
            state.success_message = None;

            match logic::validation::parse_date(&state.date) {
                Ok(date) => match logic::validation::parse_time(&state.time) {
                    Ok(time) => match logic::validation::parse_distance(&state.distance) {
                        Ok(distance) => {
                            let note = if state.note.is_empty() {
                                None
                            } else {
                                Some(state.note.clone())
                            };

                            match models::run::Run::new(date, time, distance, note) {
                                Ok(mut run) => {
                                    let result = if let Some(id) = state.editing_run_id {
                                        // Update existing run
                                        run.id = Some(id);
                                        db::queries::update_run(conn, &run)
                                            .map(|_| "Run updated successfully!".to_string())
                                    } else {
                                        // Insert new run
                                        db::queries::insert_run(conn, &run)
                                            .map(|_| "Run logged successfully!".to_string())
                                    };

                                    match result {
                                        Ok(success_msg) => {
                                            if state.editing_run_id.is_some() {
                                                // After edit, clear edit mode
                                                state.editing_run_id = None;
                                            } else {
                                                // After insert, clear fields for next entry
                                                state.distance.clear();
                                                state.note.clear();
                                                state.focused_field = InputField::Distance;
                                            }
                                            state.success_message = Some(success_msg);
                                        }
                                        Err(e) => {
                                            state.error_message = Some(format!("Database error: {}", e));
                                        }
                                    }
                                }
                                Err(e) => {
                                    state.error_message = Some(e.to_string());
                                }
                            }
                        }
                        Err(e) => {
                            state.error_message = Some(e.to_string());
                        }
                    },
                    Err(e) => {
                        state.error_message = Some(e.to_string());
                    }
                },
                Err(e) => {
                    state.error_message = Some(e.to_string());
                }
            }

            // After handling Enter key, check if we need to reload and switch screens
            if state.editing_run_id.is_none() && state.success_message.is_some() {
                // Check if we just saved an edit (success message is set but not in edit mode anymore)
                // We need to check if the success message indicates an edit
                if let Some(ref msg) = state.success_message {
                    if msg.contains("updated") {
                        // Drop the state reference to allow borrowing app mutably
                        let _ = state;
                        if let Err(e) = load_runs(app, conn) {
                            app.quick_entry_state.error_message = Some(format!("Failed to reload runs: {}", e));
                        } else {
                            app.switch_to_screen(Screen::RunList);
                        }
                        return Ok(());
                    }
                }
            }
        }
        KeyCode::Char(c) => {
            state.current_input_mut().push(c);
        }
        KeyCode::Backspace => {
            state.current_input_mut().pop();
        }
        _ => {}
    }

    Ok(())
}

fn handle_run_list_input(app: &mut App, key: KeyEvent, conn: &Connection) -> Result<()> {
    match key.code {
        KeyCode::Up => {
            let state = &mut app.run_list_state;
            if state.selected_index > 0 {
                state.selected_index -= 1;
                if state.selected_index < state.scroll_offset {
                    state.scroll_offset = state.selected_index;
                }
            }
        }
        KeyCode::Down => {
            let state = &mut app.run_list_state;
            if state.selected_index + 1 < state.runs.len() {
                state.selected_index += 1;
                if state.selected_index >= state.scroll_offset + 10 {
                    state.scroll_offset = state.selected_index - 9;
                }
            }
        }
        KeyCode::Char('e') => {
            // Edit selected run
            let state = &app.run_list_state;
            if !state.runs.is_empty() && state.selected_index < state.runs.len() {
                let run = state.runs[state.selected_index].clone();
                app.quick_entry_state.load_run(&run);
                app.switch_to_screen(Screen::QuickEntry);
            }
        }
        KeyCode::Char('d') => {
            // Delete selected run
            let run_id = {
                let state = &app.run_list_state;
                if !state.runs.is_empty() && state.selected_index < state.runs.len() {
                    state.runs[state.selected_index].id
                } else {
                    None
                }
            };

            if let Some(id) = run_id {
                db::queries::delete_run(conn, id)?;
                // Reload the run list
                load_runs(app, conn)?;
                // Adjust selected index if needed
                let state = &mut app.run_list_state;
                if state.selected_index >= state.runs.len() && !state.runs.is_empty() {
                    state.selected_index = state.runs.len() - 1;
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn load_runs(app: &mut App, conn: &Connection) -> Result<()> {
    let runs = db::queries::get_all_runs(conn)?;
    app.run_list_state.runs = runs;
    app.run_list_state.selected_index = 0;
    app.run_list_state.scroll_offset = 0;
    Ok(())
}

fn load_analytics(app: &mut App, conn: &Connection) -> Result<()> {
    let runs = db::queries::get_all_runs(conn)?;
    let analytics = logic::streak::calculate_analytics(&runs);
    app.analytics_state.analytics = analytics;
    Ok(())
}
