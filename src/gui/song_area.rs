use crate::gui::gui::AppState;
use crate::gui::song_tab::SelectedTab;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Color;
use ratatui::widgets::{Block, Borders, Tabs, Widget};
use strum::IntoEnumIterator;

pub fn song_area(f: &mut Frame, area: Rect, app: &mut AppState) {
    //create tabs
    let titles = SelectedTab::iter().map(SelectedTab::title);
    let highlight_style = (Color::Black, Color::White);
    let selected_tab_index = app.selected_tab as usize;
    let tabs = Tabs::new(titles)
        .block(Block::bordered().title("Songs").borders(Borders::ALL))
        .highlight_style(highlight_style)
        .select(selected_tab_index)
        .padding("", "")
        .divider(" | ");
    // let layout = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints(vec![Constraint::Percentage(100)])
    //     .split(area);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);
    for i in 1..100 {
        for i in 1..100 {}
    }
    f.render_widget(tabs, layout[0]);
    f.render_stateful_widget(app.selected_tab, layout[1], app);
    // app.selected_tab.render_stateful_widget(layout[1], f.buffer_mut(),app);
}
