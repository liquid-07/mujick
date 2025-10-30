use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::gui::gui::AppState;

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum SelectedArea {
    LeftArea,
    RightArea,
}

pub(crate) fn handel_controls(key: KeyEvent, app_state: &mut AppState) {
    if key.kind != KeyEventKind::Press {
        return;
    }
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app_state.should_exit = true,
        KeyCode::Char('a') => {
            log::info!("switching to Left Area");
            app_state.selected_area = SelectedArea::LeftArea;
        }
        KeyCode::Char('l') => {
            log::info!("switching to right Area");
            app_state.selected_area = SelectedArea::RightArea;
        }
        KeyCode::Down => match app_state.selected_area {
            SelectedArea::LeftArea => {
                log::info!("the index is {}", app_state.music_filter_type_index);
                if app_state.music_filter_type_index == app_state.music_filter_type.len() - 1 {
                    app_state.music_filter_type_index = 0;
                } else {
                    app_state.music_filter_type_index = app_state.music_filter_type_index + 1;
                }
            }
            SelectedArea::RightArea => {
                if app_state.music_filter_type_index == app_state.music_filter_type.len() - 1 {
                    app_state.music_filter_type_index = 0;
                } else {
                    app_state.music_filter_type_index = app_state.music_filter_type_index + 1;
                }
            }
        },
        KeyCode::Up => {
            if app_state.music_filter_type_index > 0 {
                app_state.music_filter_type_index = app_state.music_filter_type_index - 1;
            }
        }
        KeyCode::Left => app_state.selected_tab = app_state.selected_tab.previous(),
        KeyCode::Right => app_state.selected_tab = app_state.selected_tab.next(),
        _ => {}
    }
}
