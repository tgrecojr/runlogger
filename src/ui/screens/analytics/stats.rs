use crate::app::AnalyticsState;
use crate::ui::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

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

    let total_text = vec![
        Line::from(vec![
            Span::styled("Total Runs: ", Style::default().fg(Color::Gray)),
            Span::styled(state.analytics.total_runs.to_string(), theme::stat_style()),
        ]),
        Line::from(vec![
            Span::styled("Total Distance: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.2} mi", state.analytics.total_distance),
                theme::stat_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("Average: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.2} mi", state.analytics.average_distance),
                theme::stat_style(),
            ),
        ]),
    ];

    let total_block =
        Paragraph::new(total_text).block(Block::default().borders(Borders::ALL).title("Totals"));
    f.render_widget(total_block, chunks[0]);

    let weekly_text = vec![
        Line::from(vec![
            Span::styled("This Week: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} runs", state.analytics.runs_this_week),
                theme::stat_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("Avg Distance: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.2} mi", state.analytics.average_distance_this_week),
                theme::stat_style(),
            ),
        ]),
    ];

    let weekly_block =
        Paragraph::new(weekly_text).block(Block::default().borders(Borders::ALL).title("Week"));
    f.render_widget(weekly_block, chunks[1]);

    let yearly_text = vec![
        Line::from(vec![
            Span::styled("This Month: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} runs", state.analytics.runs_this_month),
                theme::stat_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("Avg: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.2} mi", state.analytics.average_distance_this_month),
                theme::stat_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("This Year: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} runs", state.analytics.runs_this_year),
                theme::stat_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("Avg: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.2} mi", state.analytics.average_distance_this_year),
                theme::stat_style(),
            ),
        ]),
    ];

    let yearly_block =
        Paragraph::new(yearly_text).block(Block::default().borders(Borders::ALL).title("Period"));
    f.render_widget(yearly_block, chunks[2]);
}
