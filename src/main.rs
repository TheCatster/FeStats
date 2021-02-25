use crate::{
    app::{App, InputMode},
    event::{Event, Events},
    formula::{attempt_formula, retrieve_formula},
    ui::draw_main_layout,
};

use anyhow::Result;
use std::io;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

mod app;
mod event;
mod formula;
mod ui;
mod util;

fn main() -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut events = Events::new();

    let mut app = App::new("FeStats");

    loop {
        // Draw UI
        terminal.draw(|f| draw_main_layout(f, &mut app))?;
        // Handle input
        match events.next()? {
            Event::Input(input) => match app.input_mode {
                InputMode::Normal => match input {
                    Key::Char('\n') => {
                        let current_formula = *app.current_items().current_item();
                        let current_input_index = app.current_stored_input().len();
                        let inputs = retrieve_formula(current_formula);
                        let current_input = &*app.current_input_text(current_input_index);
                        let text = String::from(current_input);

                        if current_input_index == inputs.len() + 1 {
                            app.current_stored_input().drain(..);
                            app.current_input().1.drain(..);
                            app.current_input().1.push(String::from(&text));
                        }

                        app.input_mode = InputMode::Editing;
                        events.disable_exit_key();
                    }
                    Key::Left => {
                        app.on_left();
                    }
                    Key::Right => {
                        app.on_right();
                    }
                    Key::Down => {
                        app.position("next");
                    }
                    Key::Up => {
                        app.position("previous");
                    }
                    Key::Char('j') => {
                        app.position("next");
                    }
                    Key::Char('k') => {
                        app.position("previous");
                    }
                    Key::Char('h') => {
                        app.on_left();
                    }
                    Key::Char('l') => {
                        app.on_right();
                    }
                    Key::Char(c) => {
                        app.on_key(c);
                    }
                    _ => {}
                },
                InputMode::Editing => match input {
                    Key::Char('\n') => {
                        let current_formula = *app.current_items().current_item();

                        if app.current_stored_input().is_empty()
                            || &app.current_stored_input()[0] != current_formula
                        {
                            app.current_stored_input().drain(..);
                            app.current_stored_input()
                                .push(String::from(current_formula));
                        }

                        let inputs = retrieve_formula(current_formula);
                        let current_input_index = app.current_stored_input().len();
                        let current_input = &*app.current_input_text(current_input_index - 1);
                        let text = String::from(current_input);

                        app.current_stored_input().push(text);
                        app.input_mode = InputMode::Normal;
                        events.enable_exit_key();
                    }
                    Key::Char(c) => {
                        let formula_name = app.current_items().current_item().to_owned();
                        let inputs = retrieve_formula(formula_name);

                        let current_input_index = if !app.current_stored_input().is_empty()
                            && app.current_stored_input().len() - 1 < inputs.len()
                        {
                            app.current_stored_input().len() - 1
                        } else {
                            0
                        };

                        let current_input = app.current_input_text(current_input_index);
                        current_input.push(c);
                    }
                    Key::Backspace => {
                        let formula_name = app.current_items().current_item().to_owned();
                        let inputs = retrieve_formula(formula_name);

                        let current_input_index = if !app.current_stored_input().is_empty()
                            && app.current_stored_input().len() - 1 < inputs.len()
                        {
                            app.current_stored_input().len() - 1
                        } else {
                            0
                        };

                        let current_input = app.current_input_text(current_input_index);
                        current_input.pop();
                    }
                    Key::Esc => {
                        app.input_mode = InputMode::Normal;
                        events.enable_exit_key();
                    }
                    _ => {}
                },
            },
            Event::Tick => {}
        }
        if app.should_quit {
            break Ok(());
        }
    }
}
