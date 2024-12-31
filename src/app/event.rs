use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

#[derive(Debug)]
pub enum AppEvent {
    Quit,
    Tick,
    Key(KeyEvent),
}

pub struct EventHandler;

impl EventHandler {
    pub fn handle_events(&self, timeout: Duration) -> Result<AppEvent, std::io::Error> {
        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => Ok(AppEvent::Quit),
                    _ => Ok(AppEvent::Key(key)),
                },
                _ => Ok(AppEvent::Tick),
            }
        } else {
            Ok(AppEvent::Tick)
        }
    }
}

// TODO implement test for events management
#[cfg(test)]
mod tests {
    todo!();
}
