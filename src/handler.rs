use crate::{app::{App, AppResult, InputMode}, connection::get_city_name};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => match key_event.code {
            KeyCode::Char('e') => {
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('q') => {
                app.running = false; 
            }
            KeyCode::Down => {
                if app.cities.len() > 0 && app.selected_city_index < app.cities.len() - 1 {
                    app.selected_city_index += 1;
                    if let Some(name) = app.cities.get(app.selected_city_index) {
                        get_city_name(name.to_string()).await;
                    } else {
                        eprintln!("Error: City not found at index {}", app.selected_city_index);
                    }
                }
            }
            KeyCode::Up => {
                if app.cities.len() > 0 && app.selected_city_index > 0 {
                    app.selected_city_index -= 1;
                    if let Some(name) = app.cities.get(app.selected_city_index) {
                        get_city_name(name.to_string()).await;
                    } else {
                        eprintln!("Error: City not found at index {}", app.selected_city_index);
                    }
                }
            }
            KeyCode::Backspace => {
                // TODO BUG WHEN DELETING A CITY FROM DOWN TO UP
                app.delete_city(app.selected_city_index);
                if !app.cities.is_empty() {
                    if app.selected_city_index == app.cities.len() {
                        app.selected_city_index -= 1;
                    }
                    if let Some(name) = app.cities.get(app.selected_city_index) {
                        get_city_name(name.to_string()).await;
                    } else {
                        eprintln!("Error: City not found at index {}", app.selected_city_index);
                    }
                }
                
            }
            _ => {}
        },
        InputMode::Editing if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Enter => app.submit_message(),
            KeyCode::Char(to_insert) => app.enter_char(to_insert),
            KeyCode::Backspace => app.delete_char(),
            KeyCode::Left => app.move_cursor_left(),
            KeyCode::Right => app.move_cursor_right(),
            KeyCode::Esc => app.input_mode = InputMode::Normal,
            _ => {}
        },
        _ => {}

    };
    Ok(())
}
