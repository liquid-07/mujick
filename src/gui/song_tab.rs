use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Widget};
use ratatui::style::{Style, Stylize};
use ratatui::style::palette::tailwind;
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Next")]
    Tab1,
    #[strum(to_string = "Lyrics")]
    Tab2,
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self {
            Self::Tab1 => self.render_tab0(area, buf),
            Self::Tab2 => self.render_tab1(area, buf),
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

    pub fn render_tab1(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("try to do http call to get the lyrics")
            .block(Block::default().title("lyrics").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .render(area, buf);
    }

    pub fn render_tab0(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("i will put all songs here ")
            .block(Block::default().title("songs").borders(Borders::ALL))
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
