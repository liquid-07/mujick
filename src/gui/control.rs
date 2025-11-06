use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

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
    if app_state.show_popup == true {
        handel_events_on_search_popup(key, app_state);
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
        KeyCode::Char('p') => {
            app_state.show_popup = true;
            app_state.search_input.clear();
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
                if app_state.selected_music_index == app_state.music_filter_type.len() - 1 {
                    app_state.music_filter_type_index = 0;
                } else {
                    app_state.selected_music_index = app_state.selected_music_index + 1;
                }
            }
        },
        KeyCode::Up => {
            if app_state.music_filter_type_index > 0 {
                app_state.music_filter_type_index = app_state.music_filter_type_index - 1;
            }
        }
        KeyCode::Left => {
            if app_state.selected_area == SelectedArea::RightArea {
                app_state.selected_tab = app_state.selected_tab.previous();
            } else {
                // Optional: handle Left key for LeftArea if needed
                log::info!("Left key pressed in LeftArea");
            }
        }
        KeyCode::Right => {
            if app_state.selected_area == SelectedArea::RightArea {
                app_state.selected_tab = app_state.selected_tab.next()
            } else {
                log::info!("right key pressed in LeftArea")
            }
        }
        _ => {}
    }
}

fn handel_events_on_search_popup(key: KeyEvent, app_state: &mut AppState) {
    match key.code {
        KeyCode::Char(c) => {
            app_state.search_input.insert(app_state.search_cursor, c);
            app_state.search_cursor += 1;
        }
        KeyCode::Backspace => {
            if app_state.search_cursor > 0 {
                app_state.search_cursor -= 1;
                app_state.search_input.remove(app_state.search_cursor);
            }
        }
        KeyCode::Left => {
            if app_state.search_cursor > 0 {
                app_state.search_cursor -= 1;
            }
        }
        KeyCode::Right => {
            if app_state.search_cursor < app_state.search_input.len() {
                app_state.search_cursor += 1;
            }
        }
        KeyCode::Enter => {
            // Perform your search here using app_state.search_input
            log::info!("Searching for: {}", app_state.search_input);
            app_state.show_popup = false;
        }
        KeyCode::Esc => {
            app_state.show_popup = false;
        }
        _ => {}
    }
}
