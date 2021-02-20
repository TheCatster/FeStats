use crate::util::{StatefulList, TabsState};

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App<'a> {
    items: Vec<StatefulList<&'a str>>,
    pub input: Vec<(usize, Vec<String>)>,
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
            input: vec![
                (0, vec![String::new()]),
                (0, vec![String::new()]),
                (0, vec![String::new()]),
                (0, vec![String::new()]),
            ],
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

    pub fn position(&mut self, position: &str) {
        let current_input = self.current_input();
        current_input.0 = 0;
        current_input.1.drain(..);
        current_input.1.push(String::new());
        self.current_stored_input().drain(..);

        let i = match position {
            "next" => match self.items[self.tabs.index].state.selected() {
                Some(i) => {
                    if i >= self.items[self.tabs.index].items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            },
            "previous" => match self.items[self.tabs.index].state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items[self.tabs.index].items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            },
            _ => unreachable!(),
        };

        self.items[self.tabs.index].state.select(Some(i));
    }

    pub fn current_title(&mut self) -> &str {
        self.tabs.titles[self.tabs.index]
    }

    pub fn current_input(&mut self) -> &mut (usize, Vec<String>) {
        &mut self.input[self.tabs.index]
    }

    pub fn current_input_text(&mut self, index: usize) -> &mut String {
        if self.input[self.tabs.index].1.len() - 1 < index {
            self.input[self.tabs.index].1.push(String::new());
        }
        &mut self.input[self.tabs.index].1[index]
    }

    pub fn current_stored_input(&mut self) -> &mut Vec<String> {
        &mut self.entered_inputs[self.tabs.index]
    }

    pub fn current_input_text_ref(&mut self) -> String {
        self.input[self.tabs.index].1.join(" ")
    }

    pub fn current_stored_input_ref(&mut self) -> &Vec<String> {
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
            'p' => {
                self.tabs.set_index(0);
            }
            'i' => {
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
