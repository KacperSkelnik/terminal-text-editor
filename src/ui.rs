use crate::app::App;
use ratatui::{
    buffer::Buffer,
    prelude::{Alignment, Frame, Rect, Stylize, Text},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

struct Popup<'a> {
    content: Text<'a>,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().white());
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .block(block)
            .render(area, buf);
    }
}

pub fn render(app: &mut App, f: &mut Frame) {
    f.set_cursor((app.cursor_position_x + 1) as u16, (app.cursor_position_y + 1) as u16);

    f.render_widget(
        Paragraph::new(format!("{}", app))
            .block(
                Block::default()
                    .title("Text editor")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left),
        f.size(),
    );

    if app.show_context {
        let area = f.size();
        let popup_height = area.height / 3;
        let popup_area = Rect {
            x: 0,
            y: area.height - popup_height,
            width: area.width,
            height: popup_height,
        };
        let bad_popup = Popup {
            content: vec!["Ctrl + c/C - Exit".into(), "Ctrl + w/W - Write".into()].into(),
        };
        f.render_widget(bad_popup, popup_area);
    }
}
