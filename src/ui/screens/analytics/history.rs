use crate::app::AnalyticsState;
use crate::logic::streak::{format_miles, goal_met, hundredths};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// One line per day of the 14-day trend, colored by the daily-goal rule.
// @spec AN-011, AN-013
pub(super) fn render_chart(f: &mut Frame, area: Rect, state: &AnalyticsState) {
    if state.analytics.recent_trend.is_empty() {
        let empty = Paragraph::new("No data to display")
            .block(Block::default().borders(Borders::ALL).title("Last 14 Days"))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(empty, area);
        return;
    }

    let mut text_lines = vec![
        Line::from(Span::styled(
            "Daily Mileage (Green = Goal Met ≥1.0 mi)",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    // On a short panel, drop the oldest days so today always stays visible.
    let trend = &state.analytics.recent_trend;
    let day_rows = area.height.saturating_sub(4) as usize; // borders + 2 header lines
    for day_data in trend.iter().skip(trend.len().saturating_sub(day_rows)) {
        let date_str = day_data.date.format("%m/%d").to_string();
        let distance_str = format_miles(day_data.distance);

        let style = if goal_met(day_data.distance) {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else if hundredths(day_data.distance) > 0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Red)
        };

        // Create a visual bar using characters
        let bar_length = (hundredths(day_data.distance) / 10).clamp(0, 50) as usize;
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

// @spec AN-012, AN-013
pub(super) fn render_monthly_breakdown(f: &mut Frame, area: Rect, state: &AnalyticsState) {
    if state.analytics.monthly_breakdown.is_empty() {
        let empty = Paragraph::new("No data to display")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Mileage by Month"),
            )
            .style(Style::default().fg(Color::Gray));
        f.render_widget(empty, area);
        return;
    }

    let mut text_lines = vec![
        Line::from(Span::styled(
            "Monthly Totals (Green ≥50 mi, Yellow ≥25 mi)",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for month_data in &state.analytics.monthly_breakdown {
        let month_str = format!("{} {}", month_abbrev(month_data.month), month_data.year);

        // Judge the color on the total as displayed (whole tenths of a mile).
        let tenths = (month_data.total_distance * 10.0).round() as i64;
        let style = if tenths >= 500 {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else if tenths >= 250 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        text_lines.push(Line::from(vec![
            Span::styled(
                format!("{:<9}", month_str),
                Style::default().fg(Color::Gray),
            ),
            Span::styled(
                format!("{:>6} mi", format!("{}.{}", tenths / 10, tenths % 10)),
                style,
            ),
            Span::styled(
                format!(
                    "  ({} runs, {:.1} avg)",
                    month_data.run_count, month_data.average_distance
                ),
                Style::default().fg(Color::Gray),
            ),
        ]));
    }

    let monthly_block = Paragraph::new(text_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Mileage by Month"),
    );

    f.render_widget(monthly_block, area);
}

fn month_abbrev(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    }
}
