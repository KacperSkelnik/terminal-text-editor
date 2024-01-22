use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(PartialEq)]
enum Message {
    Right,
    Left,
    Up,
    Down,
    Input(char),
    Remove,
    Quit,
}

impl Message {
    fn from_key(key: KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Right => Some(Message::Right),
            KeyCode::Left => Some(Message::Left),
            KeyCode::Up => Some(Message::Up),
            KeyCode::Enter | KeyCode::Down => Some(Message::Down),
            KeyCode::Char('c') | KeyCode::Char('C') if key.modifiers == KeyModifiers::CONTROL => {
                Some(Message::Quit)
            }
            KeyCode::Char(input) => Some(Message::Input(input)),
            KeyCode::Backspace => Some(Message::Remove),
            _ => None,
        }
    }
}

fn process_update(app: &mut App, message: Message) {
    match message {
        Message::Right => app.increase_cursor_position_x(),
        Message::Left => app.decrease_cursor_position_x(),
        Message::Up => app.decrease_cursor_position_y(),
        Message::Down => app.increase_cursor_position_y(),
        Message::Input(input) => app.add_character(input),
        Message::Remove => app.remove_character(),
        Message::Quit => app.quit(),
    }
}

pub fn update(app: &mut App, key_event: KeyEvent) {
    if let Some(message) = Message::from_key(key_event) {
        process_update(app, message)
    }
}
