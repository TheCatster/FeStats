use crate::{
    app::{App, InputMode},
    formula::{attempt_formula, retrieve_formula},
};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
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

    draw_body(f, app, chunks[1]);
}

fn draw_body<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(app.current_title());
    f.render_widget(block, area);

    draw_list(f, app, chunks[0]);
    draw_output(f, app, chunks[1]);
}

fn draw_output<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let formula_name = app.current_items().current_item().to_owned();
    let inputs = retrieve_formula(formula_name);
    let outputs = &attempt_formula(
        app.current_items().current_item(),
        app.current_entered_input_paragraph().to_vec(),
        app,
    );
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(*app.current_items().current_item());
    f.render_widget(block, area);

    draw_inputs(f, app, chunks[0], inputs);
    draw_formula(f, chunks[1], &formula_name, outputs);
}

fn draw_list<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    let items: Vec<ListItem> = app
        .current_items()
        .items
        .iter()
        .map(|i| ListItem::new(Spans::from(*i)).style(Style::default()))
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunks[0], &mut app.current_items().state);
}

fn draw_formula<B>(f: &mut Frame<B>, area: Rect, formula: &str, outputs: &str)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);
    let block = Block::default().borders(Borders::ALL).title(formula);

    let mut text = Text::from(Spans::from(format!("{}", outputs)));
    let formula_output = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Output"))
        .style(Style::default())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(formula_output, chunks[0]);
}

fn draw_inputs<B>(f: &mut Frame<B>, app: &mut App, area: Rect, variables: Vec<String>)
where
    B: Backend,
{
    let paragraph = app.current_input_paragraph().to_owned();
    let input = Paragraph::new(paragraph)
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
                area.x + app.current_input().width() as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
            )
        }
    }
}

// fn draw_messages<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let messages: Vec<ListItem> = app
//         .messages
//         .iter()
//         .enumerate()
//         .map(|(i, m)| {
//             let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
//             ListItem::new(content)
//         })
//         .collect();
//     let messages =
//         List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
//     f.render_widget(messages, area);
// }
