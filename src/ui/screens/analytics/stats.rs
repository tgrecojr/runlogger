use crate::app::AnalyticsState;
use crate::ui::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Totals, then the rolling windows — titled so they are not mistaken for
/// calendar weeks or months.
// @spec AN-010
pub(super) fn render(f: &mut Frame, area: Rect, state: &AnalyticsState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);
    let a = &state.analytics;

    let totals = vec![
        stat("Total Runs: ", a.total_runs.to_string()),
        stat("Total Distance: ", format!("{:.2} mi", a.total_distance)),
        stat("Average: ", format!("{:.2} mi", a.average_distance)),
    ];
    panel(f, chunks[0], "Totals", totals);

    panel(
        f,
        chunks[1],
        "Last 7 Days",
        window(a.runs_this_week, a.average_distance_this_week),
    );

    let mut month_and_year = vec![heading("Last 30 Days")];
    month_and_year.extend(window(a.runs_this_month, a.average_distance_this_month));
    month_and_year.push(heading("This Year"));
    month_and_year.extend(window(a.runs_this_year, a.average_distance_this_year));
    panel(f, chunks[2], "Last 30 Days / Year", month_and_year);
}

fn panel(f: &mut Frame, area: Rect, title: &'static str, lines: Vec<Line<'static>>) {
    let block = Block::default().borders(Borders::ALL).title(title);
    f.render_widget(Paragraph::new(lines).block(block), area);
}

/// Run count and average distance for one window.
fn window(runs: u32, average: f64) -> Vec<Line<'static>> {
    vec![
        stat("Runs: ", runs.to_string()),
        stat("Avg: ", format!("{average:.2} mi")),
    ]
}

fn stat(label: &'static str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(label, Style::default().fg(Color::Gray)),
        Span::styled(value, theme::stat_style()),
    ])
}

fn heading(text: &'static str) -> Line<'static> {
    Line::from(Span::styled(text, Style::default().fg(Color::Cyan)))
}
