use crate::audio::AudioDetails;
use log::log;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget};

///This view will contain song details
///  'apple'
pub struct SongView<'a> {
    audio_details_list: &'a [AudioDetails],
    selected: usize,
}

const SELECTED_STYLE: Style = Style::new()
    .bg(Color::White)
    .fg(Color::Black)
    .add_modifier(Modifier::BOLD);

impl<'a> SongView<'a> {
    pub fn new(audio_details_list: &'a [AudioDetails], selected: usize) -> Self {
        Self {
            audio_details_list,
            selected,
        }
    }
    fn format_duration(duration: f64) -> String {
        let minutes: u64 = (duration / 60.0) as u64;
        let seconds: u64 = (duration % 60.0) as u64;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

impl<'a> StatefulWidget for SongView<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        log::info!("i am getting called");
        let block = Block::default()
            .title("songs list")
            .borders(Borders::ALL)
            .border_style(Color::White);
        let inner_area = block.inner(area);
        let available_width = inner_area.width.saturating_sub(2) as usize;
        let current_selected = self.selected;

        let item: Vec<ListItem> = self
            .audio_details_list
            .iter()
            .enumerate()
            .map(|(i, song)| {
                let is_selected = i == current_selected;
                let checkbox = if is_selected { "[x]" } else { "[ ]" };
                let duration_str = Self::format_duration(song.duration);
                let duration_width = duration_str.len();
                let spacing = available_width
                    .saturating_sub(song.title.len() + duration_width + 6)
                    .max(0);
                let white_style = Style::default().fg(Color::White);
                let title_line = Line::from(vec![
                    Span::styled(checkbox, white_style),
                    Span::raw(" "),
                    Span::styled(&song.title, white_style),
                    Span::raw(" ".repeat(spacing)),
                    Span::styled(duration_str, white_style),
                ]);

                let artist_line = Line::from(vec![
                    Span::raw(" ".repeat(4)),
                    Span::styled(&song.artist, white_style),
                ]);
                let border_line =
                    Line::from(Span::styled("─".repeat(available_width), white_style));

                ListItem::new(vec![title_line, artist_line, border_line]).style(white_style)
            })
            .collect();

        let list = List::new(item)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">> ");

        list.render(area, buf, state);
    }
}
