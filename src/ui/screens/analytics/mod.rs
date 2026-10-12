mod history;
mod stats;
mod streaks;

use crate::app::AnalyticsState;
use crate::ui::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Width of the Mileage by Month panel: enough for its longest line.
const MONTHLY_PANEL_WIDTH: u16 = 46;

// @spec AN-011, AN-012, AN-014
pub fn render(f: &mut Frame, area: Rect, state: &AnalyticsState) {
    if let Some(ref error) = state.error_message {
        render_error(f, area, error);
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(6), // Streaks
            Constraint::Length(8), // Stats
            Constraint::Min(0),    // Last 14 Days beside Mileage by Month
        ])
        .split(area);
    let history = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(MONTHLY_PANEL_WIDTH)])
        .split(rows[2]);

    streaks::render(f, rows[0], state);
    stats::render(f, rows[1], state);
    history::render_chart(f, history[0], state);
    history::render_monthly_breakdown(f, history[1], state);
}

/// Replaces every panel: no figures are shown from a history that could not be read.
fn render_error(f: &mut Frame, area: Rect, error: &str) {
    let panel = Paragraph::new(error)
        .style(theme::error_style())
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title("Analytics"));
    f.render_widget(panel, area);
}

#[cfg(test)]
mod tests;
