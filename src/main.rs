use crate::{
    app::{App, InputMode},
    event::{Event, Events},
    formula::attempt_formula,
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
                    Key::Char('i') => {
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
                        app.current_items().position("next");
                    }
                    Key::Up => {
                        app.current_items().position("previous");
                    }
                    Key::Char('j') => {
                        app.current_items().position("next");
                    }
                    Key::Char('k') => {
                        app.current_items().position("previous");
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
                        if app.current_entered_input().len() == 0
                            || &app.current_entered_input()[0] != current_formula
                        {
                            app.current_entered_input()
                                .push(String::from(current_formula));
                        }
                        let text = String::from(app.current_input_paragraph()); //.drain(..).collect();
                        app.current_entered_input().push(text);
                        app.input_mode = InputMode::Normal;
                        events.enable_exit_key();
                    }
                    Key::Char(c) => {
                        app.current_input().push(c);
                    }
                    Key::Backspace => {
                        app.current_input().pop();
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
