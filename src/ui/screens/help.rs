use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

// @spec NAV-015, NAV-016
pub fn render(f: &mut Frame, area: Rect, db_location: Option<&str>) {
    let mut help_text = vec![
        Line::from(""),
        heading(
            "Run Logger - Help",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
    ];
    for section in [
        navigation(),
        quick_entry(),
        run_list(),
        goal(),
        data_location(db_location),
    ] {
        help_text.extend(section);
        help_text.push(Line::from(""));
    }

    let help_block = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: false });

    f.render_widget(help_block, area);
}

fn heading(text: &'static str, style: Style) -> Line<'static> {
    Line::from(Span::styled(text, style))
}

fn section(title: &'static str, lines: &[&'static str]) -> Vec<Line<'static>> {
    let mut out = vec![heading(title, Style::default().fg(Color::Cyan))];
    out.extend(lines.iter().map(|l| Line::from(*l)));
    out
}

fn navigation() -> Vec<Line<'static>> {
    section(
        "Navigation:",
        &[
            "  [Esc] then [1/2/3] - Quick Entry / Run List / Analytics (works everywhere)",
            "  [Esc] then [h] or [?] - This help screen (works everywhere)",
            "  [1-3], [h], [?] - Same, directly (not in Quick Entry, where keys type text)",
            "  [q] - Quit (not in Quick Entry)",
            "  [Ctrl+Q] or [Ctrl+C] - Quit from anywhere",
        ],
    )
}

fn quick_entry() -> Vec<Line<'static>> {
    section(
        "Quick Entry Screen:",
        &[
            "  [Tab] / [Shift+Tab] - Next / previous field",
            "  [Enter] - Save the run    [Backspace] - Delete last character",
            "  [Esc] [Esc] - Clear the form (also cancels an edit)",
            "  Date and time are pre-filled; Time resets to now after each save",
            "  Runs cannot be dated or timed in the future",
        ],
    )
}

fn run_list() -> Vec<Line<'static>> {
    section(
        "Run List Screen:",
        &[
            "  [Up/Down] - Select a run",
            "  [e] - Edit the selected run (opening Run List or Analytics cancels it)",
            "  [d] - Delete the selected run (no confirmation)",
        ],
    )
}

fn goal() -> Vec<Line<'static>> {
    let mut lines = vec![heading(
        "Your Goal:",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )];
    lines.extend(
        [
            "  Run at least 1 mile every day; runs on the same day add up.",
            "  Your streak counts through yesterday until today's mile is logged.",
        ]
        .map(Line::from),
    );
    lines
}

/// The database actually in use, as opened at startup.
fn data_location(db_location: Option<&str>) -> Vec<Line<'static>> {
    vec![
        heading("Data Location:", Style::default().fg(Color::Cyan)),
        Line::from(format!("  {}", db_location.unwrap_or("(not opened)"))),
    ]
}
