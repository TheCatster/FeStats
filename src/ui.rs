use crate::{
    app::{App, InputMode},
    formula::{attempt_formula, retrieve_formula},
};

use {
    anyhow::Result,
    std::convert::TryInto,
    tui::{
        backend::Backend,
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        text::{Span, Spans, Text},
        widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
        Frame,
    },
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

fn draw_output<B>(f: &mut Frame<B>, app: &mut App, area: Rect) -> Result<()>
where
    B: Backend,
{
    let formula_name = app.current_items().current_item().to_owned();
    let current_inputs = app.current_stored_input_ref();
    let inputs = retrieve_formula(formula_name);
    let outputs = &attempt_formula(
        app.current_items().current_item(),
        &app.current_stored_input(),
    )?;
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
    Ok(())
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

    let mut text = Text::from(outputs);
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
    let mut constraints: Vec<Constraint> = vec![]; // Constraint::Percentage(100)

    for x in &variables {
        constraints.push(Constraint::Percentage(
            (100 / &variables.len()).try_into().unwrap(),
        ));
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.as_ref())
        .split(area);

    let current_input_index = if !app.current_stored_input().is_empty()
        && app.current_stored_input().len() - 1 < constraints.len()
    {
        app.current_stored_input().len() - 1
    } else {
        0
    };

    for y in &variables {
        let index = &variables.iter().position(|x| x == y).unwrap();
        let paragraph = app.current_input_text(*index).to_owned();
        let input = Paragraph::new(paragraph)
            .style(match app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => {
                    if &current_input_index == index {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    }
                }
            })
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title(y.as_str()));
        f.render_widget(input, chunks[*index]);
    }

    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            select_next_input(f, app, chunks[current_input_index], current_input_index);
        }
    }
}

pub fn select_next_input<B>(f: &mut Frame<B>, app: &mut App, area: Rect, input_index: usize)
where
    B: Backend,
{
    f.set_cursor(
        // Put cursor past the end of the input text
        area.x + app.current_input_text(input_index).width() as u16 + 1,
        // Move one line down, from the border to the input line
        area.y + 1,
    )
}
