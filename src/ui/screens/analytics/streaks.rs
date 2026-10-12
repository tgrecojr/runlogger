use crate::app::AnalyticsState;
use crate::models::analytics::YEAR_GOAL_DAYS;
use crate::ui::theme;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// @spec AN-009, AN-016
pub(super) fn render(f: &mut Frame, area: Rect, state: &AnalyticsState) {
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
        "Goal Complete! 🎉 (100.0%)".to_string()
    } else {
        format!(
            "{} days ({:.1}%)",
            state.analytics.days_remaining_to_year_goal,
            state.analytics.year_goal_completion_percentage
        )
    };

    let text = vec![
        current_streak_line(state, streak_style),
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
                format!("Year Goal ({YEAR_GOAL_DAYS} days @ 1mi): "),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(days_remaining_text, days_remaining_style),
        ]),
    ];

    let streak_block =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Streaks"));

    f.render_widget(streak_block, area);
}

/// The current streak, plus a reminder while today's mile is still owed.
fn current_streak_line(state: &AnalyticsState, streak_style: Style) -> Line<'static> {
    let analytics = &state.analytics;
    let mut spans = vec![
        Span::styled("Current Streak: ", Style::default().fg(Color::Cyan)),
        Span::styled(format!("{} days", analytics.current_streak), streak_style),
    ];
    if analytics.current_streak > 0 && !analytics.today_goal_met {
        spans.push(Span::styled(
            " (run today to extend)",
            Style::default().fg(Color::Yellow),
        ));
    }
    Line::from(spans)
}
