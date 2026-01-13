use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Run Logger - Help",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Navigation:",
            Style::default().fg(Color::Cyan),
        )),
        Line::from("  [Esc] then [1/2/3] - Switch screens (works from anywhere)"),
        Line::from("    - Esc → 1: Quick Entry"),
        Line::from("    - Esc → 2: Run List"),
        Line::from("    - Esc → 3: Analytics"),
        Line::from("  [1-3] - Switch screens (when NOT in Quick Entry)"),
        Line::from("  [h] or [?] - This help screen (works from any screen)"),
        Line::from("  [q] - Quit application (from Help, Run List, Analytics)"),
        Line::from("  [Ctrl+Q] or [Ctrl+C] - Quit from anywhere including Quick Entry"),
        Line::from(""),
        Line::from(Span::styled(
            "Quick Entry Screen:",
            Style::default().fg(Color::Cyan),
        )),
        Line::from("  [Tab] - Move to next field"),
        Line::from("  [Shift+Tab] - Move to previous field"),
        Line::from("  [Enter] - Submit run entry"),
        Line::from("  [Esc] [Esc] - Clear all fields (press Escape twice)"),
        Line::from("  [Esc] [1/2/3] - Switch screens without leaving Quick Entry"),
        Line::from("  Type numbers/letters directly in the focused field"),
        Line::from("  [Backspace] - Delete last character"),
        Line::from(""),
        Line::from(Span::styled(
            "Run List Screen:",
            Style::default().fg(Color::Cyan),
        )),
        Line::from("  [Up/Down Arrow] - Navigate through runs"),
        Line::from("  [e] - Edit the selected run"),
        Line::from("  [d] - Delete the selected run"),
        Line::from(""),
        Line::from(Span::styled(
            "Your Goal:",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  Run at least 1 mile every day!"),
        Line::from("  Multiple runs on the same day count toward your goal."),
        Line::from(""),
        Line::from(Span::styled("Tips:", Style::default().fg(Color::Yellow))),
        Line::from("  - Date and time are pre-filled with current values"),
        Line::from("  - Just enter distance and optionally a note"),
        Line::from("  - Distance must be positive (e.g., 3.5 for 3.5 miles)"),
        Line::from("  - Your streak counts consecutive days with 1+ mile total"),
        Line::from(""),
        Line::from(Span::styled(
            "Data Location:",
            Style::default().fg(Color::Cyan),
        )),
        Line::from("  ~/Library/Application Support/runlogger/runs.db"),
    ];

    let help_block = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: false });

    f.render_widget(help_block, area);
}
