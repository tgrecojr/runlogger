//! Tests for Quick Entry screen rendering.

use crate::app::{App, InputField};
use crate::test_support::{buffer_text, draw_app, locate};
use ratatui::buffer::Buffer;
use ratatui::style::{Color, Modifier};

fn draw(app: &mut App) -> Buffer {
    draw_app(app, 80, 32)
}

fn form_app() -> App {
    let mut app = App::new();
    let state = &mut app.quick_entry_state;
    state.date = "2026-10-01".into();
    state.time = "07:00:00".into();
    state.distance = "3.5".into();
    state.note = "tempo".into();
    app
}

// @spec QE-005
#[test]
fn fields_are_labeled() {
    let text = buffer_text(&draw(&mut form_app()));
    for label in [
        "Date (YYYY-MM-DD or MM/DD/YYYY)",
        "Time (HH:MM or h:MM AM/PM)",
        "Distance (miles)",
        "Note (optional)",
    ] {
        assert!(text.contains(label), "{label:?} missing:\n{text}");
    }
}

// @spec QE-005
#[test]
fn focused_field_has_yellow_border_bold_yellow_text_and_cursor() {
    let mut app = form_app();
    app.quick_entry_state.focused_field = InputField::Distance;
    let buf = draw(&mut app);

    let (lx, ly) = locate(&buf, "Distance (miles)");
    assert_eq!(buf[(lx - 1, ly)].fg, Color::Yellow, "top-left border");
    assert_eq!(buf[(lx - 1, ly + 1)].fg, Color::Yellow, "left border");

    let (vx, vy) = locate(&buf, "3.5_");
    assert_eq!((vx, vy), (lx, ly + 1), "value sits inside the Distance box");
    for x in vx..vx + 4 {
        let cell = &buf[(x, vy)];
        assert_eq!(cell.fg, Color::Yellow, "{:?}", cell.symbol());
        assert!(
            cell.modifier.contains(Modifier::BOLD),
            "{:?}",
            cell.symbol()
        );
    }
}

// @spec QE-005
#[test]
fn unfocused_fields_have_no_cursor_or_focus_styling() {
    let mut app = form_app();
    app.quick_entry_state.focused_field = InputField::Note;
    let buf = draw(&mut app);
    let text = buffer_text(&buf);

    assert!(text.contains("tempo_"), "{text}");
    for value in ["2026-10-01", "07:00:00", "3.5"] {
        assert!(!text.contains(&format!("{value}_")), "{value}:\n{text}");
        let (x, y) = locate(&buf, value);
        assert_ne!(buf[(x, y)].fg, Color::Yellow, "{value}");
        assert!(!buf[(x, y)].modifier.contains(Modifier::BOLD), "{value}");
        assert_ne!(buf[(x - 1, y)].fg, Color::Yellow, "{value} border");
    }
}

// @spec QE-006
#[test]
fn title_reflects_edit_mode() {
    let mut app = form_app();
    let text = buffer_text(&draw(&mut app));
    assert!(text.contains("Quick Entry - Log Your Run"), "{text}");
    assert!(!text.contains("Edit Run"), "{text}");

    app.quick_entry_state.editing_run_id = Some(1);
    let text = buffer_text(&draw(&mut app));
    assert!(text.contains("Edit Run"), "{text}");
    assert!(!text.contains("Quick Entry - Log Your Run"), "{text}");
}

fn assert_message_style(buf: &Buffer, message: &str, color: Color) {
    let (x, y) = locate(buf, message);
    let (_, note_y) = locate(buf, "Note (optional)");
    let (_, note_value_y) = locate(buf, "tempo");
    assert!(
        y > note_value_y + 1 && y > note_y,
        "{message:?} not below the fields"
    );
    for dx in 0..message.chars().count() as u16 {
        let cell = &buf[(x + dx, y)];
        assert_eq!(cell.fg, color, "{:?}", cell.symbol());
        assert!(
            cell.modifier.contains(Modifier::BOLD),
            "{:?}",
            cell.symbol()
        );
    }
}

// @spec QE-021
#[test]
fn error_message_is_bold_red_below_fields() {
    let mut app = form_app();
    app.quick_entry_state.error_message = Some("Distance is required".into());
    let buf = draw(&mut app);
    assert_message_style(&buf, "Distance is required", Color::Red);
}

// @spec QE-021
#[test]
fn success_message_is_bold_green_below_fields() {
    let mut app = form_app();
    app.quick_entry_state.success_message = Some("Run logged successfully!".into());
    let buf = draw(&mut app);
    assert_message_style(&buf, "Run logged successfully!", Color::Green);
}

// @spec QE-021
#[test]
fn error_message_takes_precedence_over_success() {
    let mut app = form_app();
    app.quick_entry_state.error_message = Some("Distance is required".into());
    app.quick_entry_state.success_message = Some("Run logged successfully!".into());
    let buf = draw(&mut app);
    assert_message_style(&buf, "Distance is required", Color::Red);
    assert!(!buffer_text(&buf).contains("Run logged successfully!"));
}
