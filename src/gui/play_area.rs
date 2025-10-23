use crate::gui::gui::AppState;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn play_area(f: &mut Frame, area: Rect, app: &AppState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(40),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(30),
        ])
        .split(area);

    let equalizer = Paragraph::new("Equalizer")
        .block(Block::default().title("Equalizer").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(equalizer, layout[0]);
    let progress_bar = Paragraph::new("progress bar")
        .block(Block::default().title("progress bar").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(progress_bar, layout[1]);
    let control_area = Paragraph::new("control area")
        .block(Block::default().title("control area").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(control_area, layout[2]);

    let song_details = Paragraph::new("song details")
        .block(Block::default().title("Song details").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(song_details, layout[3]);
}
