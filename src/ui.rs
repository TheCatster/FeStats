use super::app::{App, InputMode};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

use unicode_width::UnicodeWidthStr;

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .select(app.tabs.index)
        .style(Style::default())
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_basics_tab(f, app, chunks[1]),
        1 => draw_probability_tab(f, app, chunks[1]),
        2 => draw_tests_tab(f, app, chunks[1]),
        3 => draw_regressions_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_basics_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(area);
    let block = Block::default().borders(Borders::ALL).title("Basics");
    f.render_widget(block, area);

    draw_help(f, app, chunks[0]);

    draw_input(f, app, chunks[1]);

    draw_messages(f, app, chunks[2]);
}

fn draw_probability_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);
    let block = Block::default().borders(Borders::ALL).title("Probability");
    f.render_widget(block, area);

    draw_paragraph(f, app, chunks[0]);
}

fn draw_tests_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);
    let block = Block::default().borders(Borders::ALL).title("Tests");
    f.render_widget(block, area);

    draw_paragraph(f, app, chunks[0]);
}

fn draw_regressions_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);
    let block = Block::default().borders(Borders::ALL).title("Regressions");
    f.render_widget(block, area);

    draw_paragraph(f, app, chunks[0]);
}

fn draw_help<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, area);
}

fn draw_paragraph<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let (msg, style) = (
        vec![
            Span::raw("This"),
            Span::styled(" is", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" a paragraph of"),
            Span::styled(
                " INFORMATION",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" and nothing else."),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, area);
}

fn draw_input<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, area);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                area.x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
            )
        }
    }
}

fn draw_messages<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, area);
}
