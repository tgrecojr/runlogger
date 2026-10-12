use super::*;

// @spec AN-014, AN-013
#[test]
fn analytics_error_replaces_all_panels() {
    let mut app = App::new();
    app.switch_to_screen(Screen::Analytics);
    app.analytics_state.error_message = Some("Could not load analytics: boom".into());
    let buf = draw(&mut app);
    let screen = text(&buf);

    assert_bold_red(&buf, "Could not load analytics: boom");
    for panel in [
        "Streaks",
        "Totals",
        "Daily Mileage",
        "Mileage by Month",
        "No data to display",
    ] {
        assert!(!screen.contains(panel), "{panel:?} still shown:\n{screen}");
    }
}

// @spec AN-011
#[test]
fn chart_shows_fourteen_days_colored_by_rounded_total() {
    let mut app = analytics_screen(true);
    let buf = draw(&mut app);
    let screen = text(&buf);

    assert!(screen.contains("Last 14 Days"), "{screen}");
    for day in 2..=15 {
        let label = format!("06/{day:02}");
        assert!(screen.contains(&label), "{label} missing:\n{screen}");
    }
    let met = day_distance_cell(&buf, "06/15");
    assert_eq!(
        (met.fg, met.modifier.contains(Modifier::BOLD)),
        (Color::Green, true)
    );
    assert_eq!(day_distance_cell(&buf, "06/13").fg, Color::Yellow);
    assert_eq!(day_distance_cell(&buf, "06/14").fg, Color::Red);
}

// @spec AN-011, AN-012
#[test]
fn chart_sits_beside_a_46_column_monthly_panel() {
    let mut app = analytics_screen(true);
    let buf = draw(&mut app);
    let (chart_x, chart_y) = locate(&buf, "Last 14 Days");
    let (month_x, month_y) = locate(&buf, "Mileage by Month");
    assert_eq!(chart_y, month_y, "panels are not side by side");
    assert!(month_x > chart_x);

    let line = text(&buf)
        .lines()
        .nth(month_y as usize)
        .unwrap()
        .to_string();
    let chars: Vec<char> = line.chars().collect();
    let left = (month_x - 1) as usize; // the corner before the title
    let right = (left..chars.len()).find(|&i| chars[i] == '┐').unwrap();
    assert_eq!(right - left + 1, 46, "monthly panel width");
}

// @spec AN-016
#[test]
fn streak_shows_run_today_hint_until_today_is_goal_met() {
    let mut pending = analytics_screen(false);
    let buf = draw(&mut pending);
    assert_eq!(buf[locate(&buf, "(run today to extend)")].fg, Color::Yellow);

    let mut done = analytics_screen(true);
    assert!(!text(&draw(&mut done)).contains("(run today to extend)"));
}

// @spec AN-011, AN-012
#[test]
fn short_terminal_keeps_today_and_the_full_monthly_header() {
    let mut app = analytics_screen(true);
    let mut terminal = Terminal::new(TestBackend::new(80, 30)).unwrap();
    terminal.draw(|f| render(f, &mut app)).unwrap();
    let screen = text(terminal.backend().buffer());

    assert!(screen.contains("06/15"), "today missing:\n{screen}");
    assert!(
        !screen.contains("06/02"),
        "oldest day shown instead of newest:\n{screen}"
    );
    assert!(
        screen.contains("Monthly Totals (Green ≥50 mi, Yellow ≥25 mi)"),
        "{screen}"
    );
}

// @spec AN-011, AN-001
#[test]
fn chart_distance_agrees_with_goal_status() {
    let mut app = analytics_screen(true);
    let buf = draw(&mut app);
    let screen = text(&buf);
    let line = screen.lines().find(|l| l.contains("06/12")).unwrap();
    assert!(line.contains("1.00 mi"), "{line}");
    assert_eq!(day_distance_cell(&buf, "06/12").fg, Color::Green);
}
