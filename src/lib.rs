// remember to follow ELM architecture

// Model - application state
// Update - take in input and current Model and produce a new Model
// View - displays the model to the user

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{block::Title, Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

pub struct SfPlanner {}

impl SfPlanner {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn view(model: &SfPlanner, frame: &mut Frame) {
    let sf_orange = Color::Rgb(0xFA, 0x95, 0x49);
    let sf_grey = Color::Rgb(0x5F, 0x66, 0x8C);

    frame.render_widget(Paragraph::new(format!("Hello, world!")), frame.area());

    // vertical layout first
    // split the screen into predominantly a large titlebar and lower bar
    // app content in between
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    // designate middle chunk as secondary layout
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    // titlebar
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(sf_orange))
        .border_type(BorderType::Thick)
        .title_alignment(Alignment::Right)
        .style(Style::default().bg(sf_grey));

    let title = Paragraph::new(Text::styled(
        "Satisfactory Planner v0.1",
        Style::default().fg(sf_orange),
    ))
    .block(header_block);

    // render titlebar
    frame.render_widget(title, chunks[0]);

    // list items
    let items_title = Title::from(" Items ".bold());

    let items_block = Block::default()
        .title(items_title.alignment(Alignment::Left))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(sf_orange))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(sf_grey));

    let mut list_items = Vec::<ListItem>::new();

    let items = ["Iron Ingot", "Iron Plate", "Iron Bar", "Screw"];

    for item in items {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}", item),
            Style::default().fg(sf_orange),
        ))));
    }

    let list = List::new(list_items).block(items_block);

    frame.render_widget(list, content_chunks[0]);

    // bottom bar
    let footer_block = Block::default().style(Style::default().bg(sf_grey));

    let footer_content = Paragraph::new(Text::styled(
        "Press [Q] or [Escape] to quit",
        Style::default().fg(sf_orange),
    ))
    .block(footer_block);

    frame.render_widget(footer_content, chunks[2]);
}
