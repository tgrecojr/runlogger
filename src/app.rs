use crate::logic::validation;
use crate::models::analytics::Analytics;
use crate::models::run::Run;
use chrono::Local;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    QuickEntry,
    RunList,
    Analytics,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputField {
    Date,
    Time,
    Distance,
    Note,
}

pub struct QuickEntryState {
    pub date: String,
    pub time: String,
    pub distance: String,
    pub note: String,
    pub focused_field: InputField,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub editing_run_id: Option<i64>,
}

impl QuickEntryState {
    pub fn new() -> Self {
        let now = Local::now().naive_local();
        Self {
            date: validation::format_date(&now.date()),
            time: validation::format_time(&now.time()),
            distance: String::new(),
            note: String::new(),
            focused_field: InputField::Distance,
            error_message: None,
            success_message: None,
            editing_run_id: None,
        }
    }

    pub fn clear(&mut self) {
        let now = Local::now().naive_local();
        self.date = validation::format_date(&now.date());
        self.time = validation::format_time(&now.time());
        self.distance.clear();
        self.note.clear();
        self.focused_field = InputField::Distance;
        self.error_message = None;
        self.success_message = None;
        self.editing_run_id = None;
    }

    pub fn load_run(&mut self, run: &Run) {
        self.date = validation::format_date(&run.date);
        self.time = validation::format_time(&run.time_started);
        self.distance = run.distance_miles.to_string();
        self.note = run.note.clone().unwrap_or_default();
        self.focused_field = InputField::Distance;
        self.error_message = None;
        self.success_message = None;
        self.editing_run_id = run.id;
    }

    pub fn is_editing(&self) -> bool {
        self.editing_run_id.is_some()
    }

    pub fn next_field(&mut self) {
        self.focused_field = match self.focused_field {
            InputField::Date => InputField::Time,
            InputField::Time => InputField::Distance,
            InputField::Distance => InputField::Note,
            InputField::Note => InputField::Date,
        };
    }

    pub fn prev_field(&mut self) {
        self.focused_field = match self.focused_field {
            InputField::Date => InputField::Note,
            InputField::Time => InputField::Date,
            InputField::Distance => InputField::Time,
            InputField::Note => InputField::Distance,
        };
    }

    pub fn current_input_mut(&mut self) -> &mut String {
        match self.focused_field {
            InputField::Date => &mut self.date,
            InputField::Time => &mut self.time,
            InputField::Distance => &mut self.distance,
            InputField::Note => &mut self.note,
        }
    }
}

pub struct RunListState {
    pub runs: Vec<Run>,
    pub selected_index: usize,
    pub scroll_offset: usize,
}

impl RunListState {
    pub fn new() -> Self {
        Self {
            runs: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
        }
    }
}

pub struct AnalyticsState {
    pub analytics: Analytics,
}

impl AnalyticsState {
    pub fn new() -> Self {
        Self {
            analytics: Analytics::empty(),
        }
    }
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub quick_entry_state: QuickEntryState,
    pub run_list_state: RunListState,
    pub analytics_state: AnalyticsState,
    pub waiting_for_nav: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::QuickEntry,
            should_quit: false,
            quick_entry_state: QuickEntryState::new(),
            run_list_state: RunListState::new(),
            analytics_state: AnalyticsState::new(),
            waiting_for_nav: false,
        }
    }

    pub fn switch_to_screen(&mut self, screen: Screen) {
        self.screen = screen;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
