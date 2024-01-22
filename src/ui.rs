use crate::app::App;
use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render(app: &mut App, f: &mut Frame) {
    f.set_cursor((app.cursor_position_x + 1) as u16, (app.cursor_position_y + 1) as u16);

    f.render_widget(
        Paragraph::new(format!("{}", app))
            .block(
                Block::default()
                    .title("Text editor App")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left),
        f.size(),
    );
}
