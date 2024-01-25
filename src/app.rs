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
        App { text: vec![Vec::new()], cursor_position_x: 0, cursor_position_y: 0, should_quit: false }
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

        if self.cursor_position_y >= lines - 1 {
            self.text.resize(self.cursor_position_y + 1, Vec::new());
        }

        let line_length = self.text[self.cursor_position_y].len();

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

    fn save_line_remove(&mut self) {
        if self.cursor_position_y < self.text.len() {
            let mut line_to_remove = self.text[self.cursor_position_y].clone();
            let previous_line_length = self.text[self.cursor_position_y - 1].len();
            self.text[self.cursor_position_y - 1].append(&mut line_to_remove);
            self.text.remove(self.cursor_position_y);
            self.cursor_position_x = previous_line_length;
        }

        self.decrease_cursor_position_y()
    }

    fn save_character_remove(&mut self) {
        if self.cursor_position_y < self.text.len()
            && self.cursor_position_x <= self.text[self.cursor_position_y].len()
            && self.cursor_position_x > 0
        {
            self.text[self.cursor_position_y].remove(self.cursor_position_x - 1);
        }

        self.decrease_cursor_position_x();
    }

    pub fn remove_character(&mut self) {
        if self.cursor_position_x == 0 && self.cursor_position_y != 0 {
            self.save_line_remove();
        } else {
            self.save_character_remove();
        }
    }

    pub fn new_line(&mut self) {
        if self.cursor_position_x >= self.text[self.cursor_position_y].len() {
            // end of the of the line
            self.text.push(Vec::new());
            self.cursor_position_x = if self.cursor_position_y + 1 < self.text.len() {
                self.text[self.cursor_position_y + 1].len()
            } else {
                0
            };
        } else {
            // new line in the middle of the text
            let line = self.text[self.cursor_position_y].clone();
            let (kto_keep_in_line, for_new_line) = line.split_at(self.cursor_position_x);
            self.text[self.cursor_position_y] = Vec::from(kto_keep_in_line);
            self.text.insert(self.cursor_position_y + 1, Vec::from(for_new_line));
            self.cursor_position_x = 0;
        }

        self.increase_cursor_position_y()
    }

    pub fn mouse_click(&mut self, x: usize, y: usize) {
        if x != 0 {
            self.cursor_position_x = x - 1
        } else {
            self.cursor_position_x = 0
        };
        if y != 0 {
            self.cursor_position_y = y - 1
        } else {
            self.cursor_position_y = 0
        };
    }
}
