use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct App {
    pub text: Vec<Vec<char>>,
    pub cursor_position_x: usize,
    pub cursor_position_y: usize,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            text: vec![vec![' ']],
            cursor_position_x: 0,
            cursor_position_y: 0,
            should_quit: false,
        }
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = self.text.clone();
        text.into_iter().try_for_each(|line| writeln!(f, "{}", line.iter().collect::<String>()))
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increase_cursor_position_x(&mut self) {
        if let Some(res) = self.cursor_position_x.checked_add(1) {
            self.cursor_position_x = res;
        }
    }

    pub fn decrease_cursor_position_x(&mut self) {
        if let Some(res) = self.cursor_position_x.checked_sub(1) {
            self.cursor_position_x = res;
        }
    }

    pub fn increase_cursor_position_y(&mut self) {
        if let Some(res) = self.cursor_position_y.checked_add(1) {
            self.cursor_position_y = res;
        }
    }

    pub fn decrease_cursor_position_y(&mut self) {
        if let Some(res) = self.cursor_position_y.checked_sub(1) {
            self.cursor_position_y = res;
        }
    }

    pub fn add_character(&mut self, character: char) {
        let lines = self.text.len();
        let line_length = self.text[self.cursor_position_y].len();

        if self.cursor_position_y >= lines - 1 {
            self.text.resize(self.cursor_position_y + 1, vec![' ']);
        }

        match self.cursor_position_x {
            x if x == line_length => self.text[self.cursor_position_y].push(character),
            x if x < line_length => self.text[self.cursor_position_y].insert(x, character),
            _ => {
                self.text[self.cursor_position_y].resize(self.cursor_position_x, ' ');
                self.text[self.cursor_position_y].push(character);
            }
        }

        self.increase_cursor_position_x();
    }

    pub fn remove_character(&mut self) {
        let lines = self.text.len();
        let line_length = self.text[self.cursor_position_y].len();

        if (self.cursor_position_y < lines || self.cursor_position_x <= line_length)
            && self.cursor_position_x > 0
            && self.cursor_position_x < line_length + 1
        {
            self.text[self.cursor_position_y].remove(self.cursor_position_x - 1);
        }

        self.decrease_cursor_position_x();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_cursor_x() {
        let mut app = App::default();
        app.increase_cursor_position_x();
        assert_eq!(app.cursor_position_x, 1);
    }

    #[test]
    fn test_app_cursor_y() {
        let mut app = App::default();
        app.decrease_cursor_position_y();
        assert_eq!(app.cursor_position_y, 0);
    }
}
