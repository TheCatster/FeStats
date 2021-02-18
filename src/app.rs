use crate::util::{StatefulList, TabsState};

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App<'a> {
    items: Vec<StatefulList<&'a str>>,
    pub input: Vec<String>,
    pub title: &'a str,
    pub input_mode: InputMode,
    pub entered_inputs: Vec<Vec<String>>,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            input: vec![String::new(), String::new(), String::new(), String::new()],
            input_mode: InputMode::Normal,
            entered_inputs: vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            should_quit: false,
            tabs: TabsState::new(vec!["Probability", "Intervals", "Tests", "Regressions"]),
            items: vec![
                StatefulList::with_items(vec![
                    "Factorial (!)",
                    "Permutations",
                    "Combinations",
                    "Normal Pdf",
                    "Normal Cdf",
                    "Inverse Normal",
                    "t Pdf",
                    "t Cdf",
                    "Inverse t",
                    "χ2 Pdf",
                    "χ2 Cdf",
                    "Inverse χ2",
                    "Binomial Pdf",
                    "Binomial Cdf",
                    "Inverse Binomial",
                    "Inverse Binomial N",
                    "F Pdf",
                    "F Cdf",
                    "Inverse F",
                    "Geometric Pdf",
                    "Geometric Cdf",
                    "Poisson Pdf",
                    "Poisson Cdf",
                ]),
                StatefulList::with_items(vec![
                    "z Interval",
                    "t Interval",
                    "2-Sample z Interval",
                    "2-Sample t Interval",
                    "1-Prop z Interval",
                    "2-Prop z Interval",
                ]),
                StatefulList::with_items(vec![
                    "z Test",
                    "t Test",
                    "2-Sample z Test",
                    "2-Sample t Test",
                    "1-Prop z Test",
                    "2-Prop z Test",
                    "χ2 GOF",
                    "χ2 2-way Test",
                    "2-Sample F Test",
                    "ANOVA",
                ]),
                StatefulList::with_items(vec![
                    "Linear Regression (mx+b)",
                    "Linear Regression (a+bx)",
                    "Median-Median Line",
                    "Quadratic Regression",
                    "Cubic Regression",
                    "Quartic Regression",
                    "Power Regression",
                    "Exponential Regression",
                    "Logarithmic Regression",
                    "Sinusoidal Regression",
                    "Logistic Regression (d=0)",
                    "Logistic Regression (d!=0)",
                    "Multiple Linear Regression",
                    "Correlation Matrix",
                ]),
            ],
        }
    }

    pub fn current_items(&mut self) -> &mut StatefulList<&'a str> {
        &mut self.items[self.tabs.index]
    }

    pub fn current_title(&mut self) -> &str {
        self.tabs.titles[self.tabs.index]
    }

    pub fn current_input(&mut self) -> &mut String {
        &mut self.input[self.tabs.index]
    }

    pub fn current_entered_input(&mut self) -> &mut Vec<String> {
        &mut self.entered_inputs[self.tabs.index]
    }

    pub fn current_input_paragraph(&mut self) -> &str {
        &self.input[self.tabs.index]
    }

    pub fn current_entered_input_paragraph(&mut self) -> &Vec<String> {
        &self.entered_inputs[self.tabs.index]
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
