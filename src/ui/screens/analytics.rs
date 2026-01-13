use crate::app::AnalyticsState;
use crate::ui::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, state: &AnalyticsState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(area);

    render_streak(f, chunks[0], state);
    render_stats(f, chunks[1], state);
    render_chart(f, chunks[2], state);
}

fn render_streak(f: &mut Frame, area: Rect, state: &AnalyticsState) {
    let streak_style = if state.analytics.current_streak > 0 {
        theme::goal_met_style()
    } else {
        theme::goal_not_met_style()
    };

    let days_remaining_style = if state.analytics.days_remaining_to_year_goal <= 0 {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else if state.analytics.days_remaining_to_year_goal < 100 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    };

    let days_remaining_text = if state.analytics.days_remaining_to_year_goal <= 0 {
        "Goal Complete! 🎉".to_string()
    } else {
        format!("{} days", state.analytics.days_remaining_to_year_goal)
    };

    let text = vec![
        Line::from(vec![
            Span::styled("Current Streak: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} days", state.analytics.current_streak),
                streak_style,
            ),
        ]),
        Line::from(vec![
            Span::styled("Longest Streak: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} days", state.analytics.longest_streak),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Year Goal (365 days @ 1mi): ",
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(days_remaining_text, days_remaining_style),
        ]),
    ];

    let streak_block =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Streaks"));

    f.render_widget(streak_block, area);
}

fn render_stats(f: &mut Frame, area: Rect, state: &AnalyticsState) {
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

    let weekly_text = vec![Line::from(vec![
        Span::styled("This Week: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{} runs", state.analytics.runs_this_week),
            theme::stat_style(),
        ),
    ])];

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
            Span::styled("This Year: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} runs", state.analytics.runs_this_year),
                theme::stat_style(),
            ),
        ]),
    ];

    let yearly_block =
        Paragraph::new(yearly_text).block(Block::default().borders(Borders::ALL).title("Period"));
    f.render_widget(yearly_block, chunks[2]);
}

fn render_chart(f: &mut Frame, area: Rect, state: &AnalyticsState) {
    if state.analytics.recent_trend.is_empty() {
        let empty = Paragraph::new("No data to display")
            .block(Block::default().borders(Borders::ALL).title("Last 30 Days"))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(empty, area);
        return;
    }

    // Show last 14 days of data with color coding
    let recent_data: Vec<_> = state
        .analytics
        .recent_trend
        .iter()
        .rev()
        .take(14)
        .rev()
        .collect();

    let mut text_lines = vec![
        Line::from(Span::styled(
            "Daily Mileage (Green = Goal Met ≥1.0 mi)",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for day_data in recent_data {
        let date_str = day_data.date.format("%m/%d").to_string();
        let distance_str = format!("{:.2} mi", day_data.distance);

        let style = if day_data.distance >= 1.0 {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else if day_data.distance > 0.0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Red)
        };

        // Create a visual bar using characters
        let bar_length = (day_data.distance * 10.0).min(50.0) as usize;
        let bar = "█".repeat(bar_length);

        text_lines.push(Line::from(vec![
            Span::styled(format!("{:<6}", date_str), Style::default().fg(Color::Gray)),
            Span::styled(format!("{:>8}", distance_str), style),
            Span::raw("  "),
            Span::styled(bar, style),
        ]));
    }

    let chart = Paragraph::new(text_lines)
        .block(Block::default().borders(Borders::ALL).title("Last 14 Days"));

    f.render_widget(chart, area);
}
