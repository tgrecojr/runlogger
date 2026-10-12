mod app;
mod db;
mod input;
mod lifecycle;
mod logic;
mod models;
#[cfg(test)]
mod test_support;
mod ui;

use anyhow::Result;
use app::App;
use lifecycle::CrosstermEvents;
use rusqlite::Connection;

// @spec NAV-001, NAV-002, NAV-003
fn main() -> Result<()> {
    lifecycle::install_panic_hook(lifecycle::restore_after_panic);

    let (conn, mut terminal) = lifecycle::start(open_database, lifecycle::enter_terminal)?;
    let mut app = App::new();

    lifecycle::with_restore(
        &mut terminal,
        |t| lifecycle::run_app(t, &mut app, &conn, &mut CrosstermEvents),
        lifecycle::restore_terminal,
    )
}

fn open_database() -> Result<Connection> {
    let db_path = db::connection::get_db_path()?;
    db::connection::init_db(&db_path)
}
