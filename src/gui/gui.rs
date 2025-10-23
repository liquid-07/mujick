use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::gui::play_area::play_area;
use crate::gui::song_area::song_area;
use crate::gui::song_tab::SelectedTab;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::Modifier;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState};
use std::io::{self, stdout};

pub type DefaultTerminal = Terminal<CrosstermBackend<io::Stdout>>;

const SELECTED_STYLE: Style = Style::new()
    .bg(Color::White)
    .fg(Color::Black)
    .add_modifier(Modifier::BOLD);

pub async fn init() -> Result<()> {
    color_eyre::install()?;
    let terminal = init_terminal()?;
    let result = run(terminal);
    restore_terminal()?;
    result
}

pub struct AppState {
    should_exit: bool,
    music_view_list: Vec<String>,
    music_view_selected_index: usize,
    items: Vec<String>,
    left_area: Rect,
    pub selected_tab: SelectedTab,
}

fn init_terminal() -> Result<DefaultTerminal> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn run(mut app: DefaultTerminal) -> Result<()> {
    let mut application_state = AppState {
        selected_tab:SelectedTab::Tab1,
        should_exit: false,
        music_view_list: vec![
            "Artist".to_string(),
            "Album".to_string(),
            "Playlist".to_string(),
            "Year".to_string(),
            "Genre".to_string(),
            "Unknown".to_string(),
        ],
        music_view_selected_index: 0,
        items: (0..50).map(|i| format!("Item {}", i)).collect(),
        left_area: Rect::default(),
    };
    while !application_state.should_exit {
        app.draw(|f| render_frame(f, &mut application_state))?;
        if let Event::Key(key) = event::read()? {
            handle_key(key, &mut application_state);
        };
    }
    Ok(())
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) {
    if key.kind != KeyEventKind::Press {
        return;
    }
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app_state.should_exit = true,
        KeyCode::Down => {
            log::info!("the index is {}", app_state.music_view_selected_index);
            if app_state.music_view_selected_index == app_state.music_view_list.len() - 1 {
                app_state.music_view_selected_index = 0;
            } else {
                app_state.music_view_selected_index = app_state.music_view_selected_index + 1;
            }
        }
        KeyCode::Up => {
            if app_state.music_view_selected_index > 0 {
                app_state.music_view_selected_index = app_state.music_view_selected_index - 1;
            }
        }
        KeyCode::Left => app_state.selected_tab = app_state.selected_tab.previous(),
        KeyCode::Right => app_state.selected_tab = app_state.selected_tab.next(),
        _ => {}
    }
}

fn render_frame(frame: &mut Frame, app: &mut AppState) {
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.area());
    render_left(frame, inner_layout[0], &app);
    render_right(frame, inner_layout[1], &app);
}

fn render_left(f: &mut Frame<'_>, frame: Rect, app: &AppState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ])
        .split(frame);
    f.render_widget(
        Paragraph::new("MUJICK")
            .block(Block::default().borders(Borders::ALL))
            .centered(),
        layout[0],
    );
    let music_view_item: Vec<ListItem> = app
        .music_view_list
        .iter()
        .map(|i| {
            ListItem::new(vec![
                Line::from(Span::styled(
                    i.clone(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    "-".repeat(10),
                    Style::default().fg(SLATE.c300),
                )),
            ])
        })
        .collect();
    let list = List::new(music_view_item)
        .block(Block::default().title("Music View").borders(Borders::ALL))
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">> ");
    let mut list_state = ListState::default();
    list_state.select(Some(app.music_view_selected_index));
    f.render_stateful_widget(list, layout[1], &mut list_state);

    let dev_line = Line::from(vec![Span::styled(
        "@mechanic",
        Style::default().fg(Color::Blue),
    )]);
    f.render_widget(
        Paragraph::new(dev_line)
            .block(Block::default().title("Dev Name").borders(Borders::ALL))
            .centered(),
        layout[2],
    );
}

fn render_right(f: &mut Frame<'_>, frame: Rect, app: &AppState) {
    let selected = &app.music_view_list[app.music_view_selected_index];
    let content = format!("Details for {}\n\nThis is where more info goes.", selected);

    let paragraph = Paragraph::new(content)
        .block(Block::default().title("Details").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    let paragraph_songlist = Paragraph::new("soemthing agas")
        .block(Block::default().title("song list").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    // f.render_widget(paragraph, frame);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(frame);
    //render play area

    play_area(f, layout[0], app);
    song_area(f, layout[1], app);
    //render song area

    // f.render_widget(paragraph, layout[0]);
    // f.render_widget(paragraph_songlist, layout[1]);
}
