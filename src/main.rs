use crossterm::event;
use ratatui::text::ToText;
use ratatui_templates::app::{App, AppResult};
use ratatui_templates::connection::{get_city_name, get_weather_forecast};
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use core::time;
use std::io;
use std::time::Duration;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui;

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::new();

    let event = EventHandler::new(1);
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    let mut tui = Tui::new(terminal, event);
    
    let _ = tui.init();
    let timeout = Duration::from_secs_f32(0.1);
    while app.running {
        let _ = tui.draw(&mut app);
        if event::poll(timeout)? {
            if let crossterm::event::Event::Key(key) = event::read()? {
                let _ = handle_key_events(key, &mut app).await;
            }
        }
    }

    let _ = tui.exit();
    // let city_name = get_city_name(String::from("bucuresti")).await;
    // let s = get_weather_forecast(city_name).await;
    // println!("{:?}", s);
    Ok(())
}
