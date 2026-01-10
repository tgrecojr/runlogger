use crate::app::RunListState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

pub fn render(f: &mut Frame, area: Rect, state: &RunListState) {
    // Split area for table and hints
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),     // Table
            Constraint::Length(3),   // Hints
        ])
        .split(area);

    if state.runs.is_empty() {
        let empty_message = Block::default()
            .borders(Borders::ALL)
            .title("Run List")
            .style(Style::default());
        let empty_text = Paragraph::new("No runs logged yet. Press Esc → 1 to add a run.")
            .block(empty_message)
            .style(Style::default().fg(Color::Gray));
        f.render_widget(empty_text, chunks[0]);
        render_hints(f, chunks[1]);
        return;
    }

    let header_cells = ["Date", "Time", "Distance (mi)", "Note"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows: Vec<Row> = state
        .runs
        .iter()
        .enumerate()
        .map(|(i, run)| {
            let date = run.date.format("%Y-%m-%d").to_string();
            let time = run.time_started.format("%H:%M:%S").to_string();
            let distance = format!("{:.2}", run.distance_miles);
            let note = run.note.as_deref().unwrap_or("");

            let style = if i == state.selected_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(date),
                Cell::from(time),
                Cell::from(distance),
                Cell::from(note),
            ])
            .style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Min(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Run List ({} runs)", state.runs.len())),
    );

    f.render_widget(table, chunks[0]);
    render_hints(f, chunks[1]);
}

fn render_hints(f: &mut Frame, area: Rect) {
    let hints = Paragraph::new(Line::from(vec![
        ratatui::text::Span::styled("[↑↓] ", Style::default().fg(Color::Yellow)),
        ratatui::text::Span::raw("Navigate  "),
        ratatui::text::Span::styled("[e] ", Style::default().fg(Color::Green)),
        ratatui::text::Span::raw("Edit  "),
        ratatui::text::Span::styled("[d] ", Style::default().fg(Color::Red)),
        ratatui::text::Span::raw("Delete  "),
        ratatui::text::Span::styled("[Esc→1/2/3] ", Style::default().fg(Color::Cyan)),
        ratatui::text::Span::raw("Switch screens"),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Actions"));

    f.render_widget(hints, area);
}
