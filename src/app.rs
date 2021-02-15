use crate::util::TabsState;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App<'a> {
    pub input: String,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            should_quit: false,
            tabs: TabsState::new(vec!["Basics", "Probability", "Tests", "Regressions"]),
        }
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'b' => {
                self.tabs.set_index(0);
            }
            'p' => {
                self.tabs.set_index(1);
            }
            't' => {
                self.tabs.set_index(2);
            }
            'r' => {
                self.tabs.set_index(3);
            }
            _ => {}
        }
    }
}
