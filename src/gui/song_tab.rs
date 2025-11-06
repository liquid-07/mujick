use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Widget};
use ratatui::style::palette::tailwind;
use ratatui::style::{Style, Stylize};
use ratatui::symbols;
use ratatui::text::Line;
use std::path::PathBuf;

use crate::audio::{AudioDetails, AudioType};
use crate::gui::gui::AppState;
use crate::gui::song_view::SongView;
use ratatui::widgets::StatefulWidget;
use ratatui::widgets::{Block, Borders, ListState, Padding, Paragraph};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Next")]
    Tab1,
    #[strum(to_string = "Lyrics")]
    Tab2,
}

impl StatefulWidget for SelectedTab {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, app_state: &mut Self::State)
    where
        Self: Sized,
    {
        match self {
            Self::Tab1 => self.render_tab0(area, buf, app_state),
            Self::Tab2 => self.render_tab1(area, buf, app_state),
        }
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub(crate) fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub(crate) fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    pub fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    pub fn render_tab0(self, area: Rect, buf: &mut Buffer, app_state: &mut AppState) {
        let songs = get_dummy_audio_details();
        let song_view: SongView = SongView::new(&songs, 0);
        let mut list_state = ListState::default();
        list_state.select(Some(app_state.selected_music_index));
        song_view.render(area, buf, &mut list_state);
        // Paragraph::new("i will put all songs here ")
        //     .block(Block::default().title("songs").borders(Borders::ALL))
        //     .style(Style::default().fg(Color::White))
        //     .render(area, buf);
    }

    pub fn render_tab1(self, area: Rect, buf: &mut Buffer, app_state: &mut AppState) {
        Paragraph::new("try to do http call to get the lyrics")
            .block(Block::default().title("lyrics").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .render(area, buf);
    }

    /// A block surrounding the tab's content
    pub fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            // Self::Tab1 => tailwind::BLUE,
            // Self::Tab2 => tailwind::EMERALD,
            _ => tailwind::BLUE,
        }
    }
}

pub fn get_dummy_audio_details() -> Vec<AudioDetails> {
    vec![
        AudioDetails::new(
            PathBuf::from("/music/Daft Punk - Get Lucky.mp3"),
            AudioType::Mp3,
            "Get Lucky".into(),
            "Daft Punk".into(),
            "Random Access Memories".into(),
            "Electronic".into(),
            "Grammy-winning hit".into(),
            369.0, // 6:09
            Some(44100),
            Some(16),
            Some(2),
            2013,
        ),
        AudioDetails::new(
            PathBuf::from("/music/Adele - Hello.flac"),
            AudioType::Flac,
            "Hello".into(),
            "Adele".into(),
            "25".into(),
            "Pop".into(),
            "Emotional ballad".into(),
            295.0, // 4:55
            Some(48000),
            Some(24),
            Some(2),
            2015,
        ),
        AudioDetails::new(
            PathBuf::from("/music/Imagine Dragons - Believer.wav"),
            AudioType::Wav,
            "Believer".into(),
            "Imagine Dragons".into(),
            "Evolve".into(),
            "Rock".into(),
            "Energetic rock anthem".into(),
            204.0, // 3:24
            Some(44100),
            Some(16),
            Some(2),
            2017,
        ),
        AudioDetails::new(
            PathBuf::from("/music/The Weeknd - Blinding Lights.ogg"),
            AudioType::Ogg,
            "Blinding Lights".into(),
            "The Weeknd".into(),
            "After Hours".into(),
            "Synthwave".into(),
            "Retro 80s vibe".into(),
            200.0, // 3:20
            Some(44100),
            Some(16),
            Some(2),
            2020,
        ),
        AudioDetails::new(
            PathBuf::from("/music/Taylor Swift - Cardigan.aac"),
            AudioType::Aac,
            "Cardigan".into(),
            "Taylor Swift".into(),
            "Folklore".into(),
            "Indie Pop".into(),
            "Soft and introspective".into(),
            239.0, // 3:59
            Some(44100),
            Some(16),
            Some(2),
            2020,
        ),
    ]
}
