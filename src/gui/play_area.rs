use crate::audio::{AudioDetails, AudioType};
use crate::gui::gui::AppState;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::style::Stylize;
use ratatui::style::palette::tailwind;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Gauge, Padding, Paragraph, Row, Table};
use std::path::PathBuf;

const ALL_LENGTH: Color = tailwind::WHITE;
const COMPLETED: Color = tailwind::BLUE.c800;

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

    // render progress bar
    render_progressbar(f, layout[1], app);

    //render control area
    render_control_area(f, layout[2], app);
    // f.render_widget(control_area, layout[2]);

    // let song_details = Paragraph::new("song details")
    //     .block(Block::default().title("Song details").borders(Borders::ALL))
    //     .style(Style::default().fg(Color::White));
    //
    // f.render_widget(song_details, layout[3]);

    render_audio_description(f, layout[3], app);
}

fn render_control_area(f: &mut Frame<'_>, layout: Rect, app: &AppState) {
    let outer = Block::default().title("Controls").borders(Borders::ALL);
    f.render_widget(outer, layout);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .margin(1)
        .split(layout);
    let prev =
        Paragraph::new(Span::styled("⏮", Style::default().bold())).alignment(Alignment::Center);
    let play =
        Paragraph::new(Span::styled("⏯️", Style::default().bold())).alignment(Alignment::Center);
    let next =
        Paragraph::new(Span::styled("⏭", Style::default().bold())).alignment(Alignment::Center);
    f.render_widget(prev, chunks[0]);
    f.render_widget(play, chunks[1]);
    f.render_widget(next, chunks[2]);
}

fn render_audio_description(f: &mut Frame, area: Rect, app: &AppState) {
    let s = AudioDetails::new(
        PathBuf::from("/music/Daft Pussnk - Get Lucky.mp3"),
        AudioType::Mp3,
        "Get Lucky".into(),
        "Daft deex".into(),
        "Random Access Memories".into(),
        "Electronic".into(),
        "Grammy-winning hit".into(),
        369.0, // 6:09
        Some(44100),
        Some(16),
        Some(2),
        2013,
    );
    let rows = vec![
        Row::new(vec![Cell::from("Title:"), Cell::from(s.title.clone())]),
        Row::new(vec![Cell::from("Artist:"), Cell::from(s.artist.clone())]),
        Row::new(vec![Cell::from("Album:"), Cell::from(s.album.clone())]),
        Row::new(vec![Cell::from("Genre:"), Cell::from(s.genre.clone())]),
        Row::new(vec![Cell::from("Comment"), Cell::from(s.comment.clone())]),
        Row::new(vec![
            Cell::from("Duration:"),
            Cell::from(format!("{:.1}s", s.duration)),
        ]),
        Row::new(vec![
            Cell::from("Sample Rate:"),
            Cell::from(s.sample_rate.map_or("-".to_string(), |v| v.to_string())),
        ]),
        Row::new(vec![
            Cell::from("Bit Depth:"),
            Cell::from(s.bit_depth.map_or("-".to_string(), |v| v.to_string())),
        ]),
        Row::new(vec![
            Cell::from("Channels:"),
            Cell::from(s.channels.map_or("-".to_string(), |v| v.to_string())),
        ]),
        Row::new(vec![Cell::from("Year:"), Cell::from(s.year.to_string())]),
    ];

    let widths = [Constraint::Length(20), Constraint::Length(30)];
    let table = Table::new(rows, widths)
        .block(Block::default().title("Song Info").borders(Borders::ALL))
        .widths(&[Constraint::Length(15), Constraint::Length(30)]);

    f.render_widget(table, area);
}

fn render_progressbar(f: &mut Frame, area: Rect, app: &AppState) {
    let title = title_block("progressbar");
    let label = Span::styled(format!("{:.1}/100", 10), Style::new().italic().bold());
    let gauge = Gauge::default()
        .block(title)
        .gauge_style(ALL_LENGTH)
        .ratio(10.0 / 100.0)
        .label(label);

    f.render_widget(gauge, area);
}

fn title_block(title: &str) -> Block<'_> {
    let title = Line::from(title).left_aligned();
    Block::new()
        .borders(Borders::ALL)
        .padding(Padding::vertical(1))
        .title(title)
        .fg(Color::White)
}
