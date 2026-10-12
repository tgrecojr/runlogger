use crate::app::RunListState;
use crate::ui::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

// @spec RL-002, RL-003, RL-004, RL-005, RL-007, RL-012, RL-013
pub fn render(f: &mut Frame, area: Rect, state: &mut RunListState) {
    // Split area for an optional error (wrapped to fit), the table and hints
    let error_rows = state
        .error_message
        .as_deref()
        .map_or(0, |e| wrapped_height(e, area.width));
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
            Paragraph::new(error.as_str())
                .style(theme::error_style())
                .wrap(Wrap { trim: false }),
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
            .style(theme::muted());
        f.render_widget(empty_text, chunks[0]);
        render_hints(f, chunks[1]);
        return;
    }

    let header_cells = ["Date", "Time", "Distance (mi)", "Note"]
        .iter()
        .map(|h| Cell::from(*h).style(theme::table_header()));
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
                theme::selected_row()
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
            .title(format!("Run List ({})", run_count(state.runs.len()))),
    );

    // The persisted TableState scrolls just enough to keep the selection in view.
    state.table_state.select(Some(state.selected_index));
    f.render_stateful_widget(table, chunks[0], &mut state.table_state);
    render_hints(f, chunks[1]);
}

fn render_hints(f: &mut Frame, area: Rect) {
    let hints = Paragraph::new(Line::from(vec![
        ratatui::text::Span::styled("[↑↓] ", theme::navigate_key()),
        ratatui::text::Span::raw("Navigate  "),
        ratatui::text::Span::styled("[e] ", theme::edit_key()),
        ratatui::text::Span::raw("Edit  "),
        ratatui::text::Span::styled("[d] ", theme::delete_key()),
        ratatui::text::Span::raw("Delete  "),
        ratatui::text::Span::styled("[Esc→1/2/3] ", theme::switch_key()),
        ratatui::text::Span::raw("Switch screens"),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Actions"));

    f.render_widget(hints, area);
}

fn run_count(n: usize) -> String {
    if n == 1 {
        "1 run".to_string()
    } else {
        format!("{n} runs")
    }
}

/// Rows `text` needs when word-wrapped to `width` columns (long words break).
fn wrapped_height(text: &str, width: u16) -> u16 {
    let width = usize::from(width.max(1));
    let (mut lines, mut col) = (1u16, 0usize);
    for word in text.split(' ') {
        let len = word.chars().count();
        let needed = if col == 0 { len } else { col + 1 + len };
        if needed <= width {
            col = needed;
        } else {
            lines += 1;
            col = len;
            while col > width {
                lines += 1;
                col -= width;
            }
        }
    }
    lines
}

#[cfg(test)]
mod tests;
