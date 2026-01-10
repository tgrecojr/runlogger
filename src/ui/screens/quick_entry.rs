use crate::app::{InputField, QuickEntryState};
use crate::ui::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render(f: &mut Frame, area: Rect, state: &QuickEntryState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().add_modifier(Modifier::BOLD));
    let title_str = if state.is_editing() {
        "Edit Run"
    } else {
        "Quick Entry - Log Your Run"
    };
    let title_text = Paragraph::new(title_str)
        .block(title)
        .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(title_text, chunks[0]);

    render_input_field(f, chunks[1], "Date (YYYY-MM-DD)", &state.date, state.focused_field == InputField::Date);
    render_input_field(f, chunks[2], "Time (HH:MM:SS)", &state.time, state.focused_field == InputField::Time);
    render_input_field(f, chunks[3], "Distance (miles)", &state.distance, state.focused_field == InputField::Distance);
    render_input_field(f, chunks[4], "Note (optional)", &state.note, state.focused_field == InputField::Note);

    if let Some(ref error) = state.error_message {
        let error_widget = Paragraph::new(error.as_str())
            .style(theme::error_style())
            .wrap(Wrap { trim: false });
        f.render_widget(error_widget, chunks[5]);
    } else if let Some(ref success) = state.success_message {
        let success_widget = Paragraph::new(success.as_str())
            .style(theme::success_style())
            .wrap(Wrap { trim: false });
        f.render_widget(success_widget, chunks[5]);
    }
}

fn render_input_field(
    f: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    is_focused: bool,
) {
    let style = if is_focused {
        theme::focused_input_style()
    } else {
        theme::unfocused_input_style()
    };

    let border_style = if is_focused {
        Style::default().fg(ratatui::style::Color::Yellow)
    } else {
        Style::default().fg(ratatui::style::Color::Gray)
    };

    let display_value = if is_focused {
        format!("{}_", value)
    } else {
        value.to_string()
    };

    let input = Paragraph::new(display_value)
        .style(style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(label)
                .border_style(border_style),
        );

    f.render_widget(input, area);
}
