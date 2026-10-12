//! Every style the UI uses, named by role. Screens never pick colors directly,
//! so a color change is made here once.

use ratatui::style::{Color, Modifier, Style};

fn fg(color: Color) -> Style {
    Style::default().fg(color)
}

fn bold(color: Color) -> Style {
    fg(color).add_modifier(Modifier::BOLD)
}

// --- Chrome: header, footer, titles, headings, muted text

pub fn active_tab() -> Style {
    bold(Color::Yellow)
}

pub fn inactive_tab() -> Style {
    fg(Color::Gray)
}

pub fn footer_hint() -> Style {
    fg(Color::DarkGray)
}

pub fn esc_menu() -> Style {
    bold(Color::Cyan)
}

/// A screen's own title line (e.g. "Quick Entry - Log Your Run").
pub fn screen_title() -> Style {
    Style::default().add_modifier(Modifier::BOLD)
}

/// The title at the top of the Help screen.
pub fn help_title() -> Style {
    bold(Color::Yellow)
}

/// Section and field labels ("Navigation:", "Current Streak: ", "This Year").
pub fn heading() -> Style {
    fg(Color::Cyan)
}

/// The explanatory first line inside a chart panel.
pub fn panel_heading() -> Style {
    bold(Color::Cyan)
}

/// Secondary text: empty-state messages, stat labels, dates, run counts.
pub fn muted() -> Style {
    fg(Color::Gray)
}

pub fn error_style() -> Style {
    bold(Color::Red)
}

pub fn success_style() -> Style {
    bold(Color::Green)
}

// --- Quick Entry form

pub fn focused_input_style() -> Style {
    bold(Color::Yellow)
}

pub fn unfocused_input_style() -> Style {
    fg(Color::White)
}

pub fn focused_border() -> Style {
    fg(Color::Yellow)
}

pub fn unfocused_border() -> Style {
    fg(Color::Gray)
}

// --- Run List

pub fn table_header() -> Style {
    bold(Color::Yellow)
}

pub fn selected_row() -> Style {
    bold(Color::Yellow)
}

pub fn navigate_key() -> Style {
    fg(Color::Yellow)
}

pub fn edit_key() -> Style {
    fg(Color::Green)
}

pub fn delete_key() -> Style {
    fg(Color::Red)
}

pub fn switch_key() -> Style {
    fg(Color::Cyan)
}

// --- Analytics: figures and goal status

/// A headline figure (totals, longest streak, a distant year goal).
pub fn stat_style() -> Style {
    bold(Color::White)
}

/// Goal met: a goal-met day, an active streak, a completed year, the goal heading.
pub fn goal_met_style() -> Style {
    bold(Color::Green)
}

/// Some distance but below the goal for that day.
pub fn goal_partial_style() -> Style {
    fg(Color::Yellow)
}

/// No distance, or no current streak.
pub fn goal_not_met_style() -> Style {
    fg(Color::Red)
}

/// The year goal is close (fewer than 100 days left).
pub fn goal_close_style() -> Style {
    bold(Color::Yellow)
}

/// A reminder that today's mile is still owed.
pub fn reminder_style() -> Style {
    fg(Color::Yellow)
}

/// Monthly totals: 50+ miles, 25+ miles, and below.
pub fn volume_high() -> Style {
    bold(Color::Green)
}

pub fn volume_mid() -> Style {
    fg(Color::Yellow)
}

pub fn volume_low() -> Style {
    fg(Color::White)
}
