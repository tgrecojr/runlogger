use super::*;

// @spec RL-012, RL-004
#[test]
fn run_list_load_error_shows_in_bold_red_without_empty_history_message() {
    let mut app = App::new();
    app.switch_to_screen(Screen::RunList);
    app.run_list_state.error_message = Some("Could not load runs: boom".into());
    let buf = draw(&mut app);

    assert_bold_red(&buf, "Could not load runs: boom");
    assert!(!text(&buf).contains("No runs logged yet"));
}

// @spec RL-004
#[test]
fn run_list_empty_history_message_shows_after_successful_load() {
    let mut app = App::new();
    app.switch_to_screen(Screen::RunList);
    assert!(text(&draw(&mut app)).contains("No runs logged yet"));
}

// @spec RL-007
#[test]
fn run_list_scrolls_to_keep_the_selection_visible() {
    let mut app = App::new();
    app.switch_to_screen(Screen::RunList);
    app.run_list_state.runs = many_runs(40);
    let date_of = |app: &App, i: usize| {
        app.run_list_state.runs[i]
            .date
            .format("%Y-%m-%d")
            .to_string()
    };

    for i in [35, 39, 20, 0] {
        app.run_list_state.selected_index = i;
        let expected = date_of(&app, i);
        let screen = text(&draw(&mut app));
        assert!(
            screen.contains(&expected),
            "selected run {i} ({expected}) not visible:\n{screen}"
        );
    }
}
