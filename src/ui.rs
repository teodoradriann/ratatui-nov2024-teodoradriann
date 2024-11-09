use std::os::macos::raw::stat;

use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, Paragraph, ListState};
use ratatui::widgets::block::Position;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::ListItem;
use ratatui::Frame;
use reqwest::tls::TlsInfo;
use crate::app::App;
use crate::app::InputMode;

pub fn render(app: &mut App, frame: &mut Frame) {

    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);

    let [help_area, input_area, info_area] = vertical.areas(frame.area());

    let info_sub_areas= Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    .split(info_area);

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                //"Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "e".bold(),
                " to start editing.".bold(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to stop editing, ".into(),
                "Enter".bold(),
                " to record the message".into(),
            ],
            Style::default(),
        ),
    };

    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, help_area);

    let input = Paragraph::new(app.input.as_str())
            .style(match app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"));

    frame.render_widget(input, input_area);

    match app.input_mode {
        InputMode::Normal => {}
        #[allow(clippy::cast_possible_truncation)]
        InputMode::Editing => frame.set_cursor(
            input_area.x + app.character_index as u16 + 1,
            input_area.y + 1,
        ),
    }

    let cities_text = Span::styled("Cities Searched", 
    Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC));

    let cities_searched: Vec<ListItem> = app.cities.iter().map(|city| ListItem::new(city.clone())).collect();
            
    let mut state = ListState::default();
    state.select(Some(app.selected_city_index));

    let cities_searched_widget = List::new(cities_searched)
        .block(Block::default().title(cities_text).borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Gray));

    frame.render_stateful_widget(cities_searched_widget, info_sub_areas[0], &mut state);

    // TODO: ADD TEXT HERE FOR THE WEATHER
    let additional_info = Paragraph::new("")
        .block(Block::default().title("Info").borders(Borders::ALL));
    frame.render_widget(additional_info, info_sub_areas[1]);
    
}