use crate::app::RunListState;
use crate::ui::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

// @spec RL-002, RL-003, RL-004, RL-005, RL-007, RL-012, RL-013
pub fn render(f: &mut Frame, area: Rect, state: &mut RunListState) {
    // Split area for an optional error line, the table and hints
    let error_rows = if state.error_message.is_some() { 1 } else { 0 };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(error_rows), // Error
            Constraint::Min(10),            // Table
            Constraint::Length(3),          // Hints
        ])
        .split(area);
    let (error_area, chunks) = (chunks[0], &chunks[1..]);

    if let Some(ref error) = state.error_message {
        f.render_widget(
            Paragraph::new(error.as_str()).style(theme::error_style()),
            error_area,
        );
    }

    if state.runs.is_empty() && state.error_message.is_none() {
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

    let header_cells = ["Date", "Time", "Distance (mi)", "Note"].iter().map(|h| {
        Cell::from(*h).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    });
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

    // The persisted TableState scrolls just enough to keep the selection in view.
    state.table_state.select(Some(state.selected_index));
    f.render_stateful_widget(table, chunks[0], &mut state.table_state);
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

#[cfg(test)]
mod tests;
