// remember to follow ELM architecture

// Model - application state
// Update - take in input and current Model and produce a new Model
// View - displays the model to the user

use std::{fs::File, path::PathBuf};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{block::Title, Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Item {
    name: String,
    description: String,
    sink_points: usize,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Ingredient {
    name: String,
    amount: usize,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Recipe {
    name: String,
    ingredients: Vec<Ingredient>,
    products: Vec<Ingredient>,
    building: String,
    duration: usize,
}

pub struct SfPlanner {
    items: Vec<Item>,
    recipes: Vec<Recipe>,
    selected_item: ListState,
    selected_recipe: Option<ListState>,
}

impl SfPlanner {
    #[must_use]
    pub fn new_from_json(items_json: PathBuf, recipes_json: PathBuf) -> Self {
        let items_file = File::open(items_json).unwrap();
        let mut items: Vec<Item> = serde_json::from_reader(items_file).unwrap();
        items.sort();

        let recipes_file = File::open(recipes_json).unwrap();
        let mut recipes: Vec<Recipe> = serde_json::from_reader(recipes_file).unwrap();
        recipes.sort();
        Self {
            items,
            recipes,
            selected_item: ListState::default(),
            selected_recipe: None,
        }
    }
}

pub enum SelectionMode {
    ItemSelect(String),
    RecipeSelect(String),
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

    let items = model.items.clone();

    for item in items {
        list_items.push(ListItem::new(Line::from(Span::styled(
            item.name,
            Style::default().fg(sf_orange),
        ))));
    }

    let list = List::new(list_items).block(items_block);

    frame.render_widget(list, content_chunks[0]);

    let mut list_recipes = Vec::<ListItem>::new();
    let recipes = model.recipes.clone();

    for recipe in recipes {
        list_recipes.push(ListItem::new(Line::from(Span::styled(
            recipe.name,
            Style::default().fg(sf_orange),
        ))));
    }

    let recipe_title = Title::from(" Recipe ".bold());

    let recipe_block = Block::default()
        .title(recipe_title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(sf_orange))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(sf_grey));

    let recipe = List::new(list_recipes).block(recipe_block);

    frame.render_widget(recipe, content_chunks[1]);

    // bottom bar
    let footer_block = Block::default().style(Style::default().bg(sf_grey));

    let footer_content = Paragraph::new(Text::styled(
        "Press [Q] or [Escape] to quit",
        Style::default().fg(sf_orange),
    ))
    .block(footer_block);

    frame.render_widget(footer_content, chunks[2]);
}
