use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

#[derive(PartialEq)]
enum Message {
    Right,
    Left,
    Up,
    Down,
    Input(char),
    Remove,
    NewLine,
    Quit,
    Click(usize, usize),
}

impl Message {
    fn from_key(key: KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Right => Some(Message::Right),
            KeyCode::Left => Some(Message::Left),
            KeyCode::Up => Some(Message::Up),
            KeyCode::Down => Some(Message::Down),
            KeyCode::Char('c') | KeyCode::Char('C') if key.modifiers == KeyModifiers::CONTROL => Some(Message::Quit),
            KeyCode::Char(input) => Some(Message::Input(input)),
            KeyCode::Backspace => Some(Message::Remove),
            KeyCode::Enter => Some(Message::NewLine),
            _ => None,
        }
    }

    fn from_mouse(mouse_event: MouseEvent) -> Option<Message> {
        match mouse_event.kind {
            MouseEventKind::Down(button) if button == MouseButton::Left => {
                Some(Message::Click(mouse_event.column as usize, mouse_event.row as usize))
            }
            _ => None,
        }
    }
}

pub fn update_for_keys(app: &mut App, key_event: KeyEvent) {
    if let Some(message) = Message::from_key(key_event) {
        match message {
            Message::Right => app.increase_cursor_position_x(),
            Message::Left => app.decrease_cursor_position_x(),
            Message::Up => app.decrease_cursor_position_y(),
            Message::Down => app.increase_cursor_position_y(),
            Message::Input(input) => app.add_character(input),
            Message::Remove => app.remove_character(),
            Message::NewLine => app.new_line(),
            Message::Quit => app.quit(),
            _ => (),
        }
    }
}

pub fn update_for_mouse(app: &mut App, mouse_event: MouseEvent) {
    if let Some(message) = Message::from_mouse(mouse_event) {
        if let Message::Click(x, y) = message {
            app.mouse_click(x, y)
        }
    }
}
